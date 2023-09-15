use salvo::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryTrait};
use sea_orm::ActiveValue::Set;

use crate::AppState;
use crate::model::{sys_role, sys_role_menu, sys_user_role};
use crate::model::prelude::{SysMenu, SysRole, SysRoleMenu, SysUserRole};
use crate::vo::{err_result_msg, ok_result_data, ok_result_msg, ok_result_page};
use crate::vo::role_vo::*;

// 查询角色列表
#[handler]
pub async fn role_list(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<RoleListReq>().await.unwrap();
    log::info!("role_list params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let paginator = SysRole::find()
        .apply_if(item.role_name.clone(), |query, v| {
            query.filter(sys_role::Column::RoleName.eq(v))
        })
        .apply_if(item.status_id.clone(), |query, v| {
            query.filter(sys_role::Column::StatusId.eq(v))
        }).paginate(conn, item.page_size.clone());

    let total = paginator.num_items().await.unwrap_or_default();


    let mut list_data: Vec<RoleListData> = Vec::new();

    for role in paginator.fetch_page(item.page_no.clone() - 1).await.unwrap_or_default() {
        list_data.push(RoleListData {
            id: role.id,
            sort: role.sort,
            status_id: role.status_id,
            role_name: role.role_name,
            remark: role.remark,
            create_time: role.create_time.to_string(),
            update_time: role.update_time.to_string(),
        })
    }

    res.render(Json(ok_result_page(list_data, total)))
}

// 添加角色信息
#[handler]
pub async fn role_save(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let role = req.parse_json::<RoleSaveReq>().await.unwrap();
    log::info!("role_save params: {:?}", &role);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let sys_role = sys_role::ActiveModel {
        id: NotSet,
        status_id: Set(role.status_id),
        sort: Set(role.sort),
        role_name: Set(role.role_name),
        remark: Set(role.remark.unwrap_or_default()),
        ..Default::default()
    };

    SysRole::insert(sys_role).exec(conn).await.unwrap();
    res.render(Json(ok_result_msg("添加角色成功!")))
}

// 更新角色信息
#[handler]
pub async fn role_update(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let role = req.parse_json::<RoleUpdateReq>().await.unwrap();
    log::info!("role_update params: {:?}", &role);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    if SysRole::find_by_id(role.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        return res.render(Json(err_result_msg("角色不存在,不能更新!")));
    }
    let sys_role = sys_role::ActiveModel {
        id: Set(role.id),
        status_id: Set(role.status_id),
        sort: Set(role.sort),
        role_name: Set(role.role_name),
        remark: Set(role.remark.unwrap_or_default()),
        ..Default::default()
    };

    SysRole::update(sys_role).exec(conn).await.unwrap();
    res.render(Json(ok_result_msg("更新角色成功!")))
}

// 删除角色信息
#[handler]
pub async fn role_delete(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<RoleDeleteReq>().await.unwrap();
    log::info!("role_delete params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let ids = item.ids.clone();

    if SysUserRole::find().filter(sys_user_role::Column::RoleId.is_in(ids)).count(conn).await.unwrap_or_default() > 0 {
        return res.render(Json(err_result_msg("角色已被使用,不能直接删除！")));
    }

    SysRole::delete_many().filter(sys_role::Column::Id.is_in(item.ids.clone())).exec(conn).await.unwrap();
    res.render(Json(ok_result_msg("删除角色信息成功!")))
}

// 查询角色关联的菜单
#[handler]
pub async fn query_role_menu(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<QueryRoleMenuReq>().await.unwrap();
    log::info!("query_role_menu params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i64> = Vec::new();

    for x in SysMenu::find().all(conn).await.unwrap_or_default() {
        menu_data_list.push(MenuDataList {
            id: x.id.clone(),
            parent_id: x.parent_id,
            title: x.menu_name.clone(),
            key: x.id.to_string(),
            label: x.menu_name,
            is_penultimate: x.parent_id == 2,
        });
        role_menu_ids.push(x.id)
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        role_menu_ids.clear();
        for x in SysRoleMenu::find().filter(sys_role_menu::Column::RoleId.eq(item.role_id.clone())).all(conn).await.unwrap_or_default() {
            role_menu_ids.push(x.menu_id);
        }
    }

    res.render(Json(ok_result_data(QueryRoleMenuData { role_menus: role_menu_ids, menu_list: menu_data_list })))
}

// 更新角色关联的菜单
#[handler]
pub async fn update_role_menu(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<UpdateRoleMenuReq>().await.unwrap();
    log::info!("update_role_menu params: {:?}", &item);
    let state = depot.obtain::<AppState>().unwrap();

    let conn = &state.conn;
    let role_id = item.role_id.clone();

    SysRoleMenu::delete_many().filter(sys_role_menu::Column::RoleId.eq(role_id)).exec(conn).await.unwrap();
    let mut menu_role: Vec<sys_role_menu::ActiveModel> = Vec::new();

    for id in &item.menu_ids {
        let menu_id = id.clone();
        menu_role.push(sys_role_menu::ActiveModel {
            id: NotSet,
            status_id: Set(1),
            sort: Set(1),
            menu_id: Set(menu_id),
            role_id: Set(role_id.clone()),
            ..Default::default()
        })
    }
    SysRoleMenu::insert_many(menu_role).exec(conn).await.unwrap();
    res.render(Json(ok_result_msg("更新角色关联的菜单!")))
}
