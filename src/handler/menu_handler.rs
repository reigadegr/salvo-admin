use salvo::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder};
use sea_orm::ActiveValue::Set;

use crate::AppState;
use crate::model::prelude::SysMenu;
use crate::model::sys_menu;
use crate::model::sys_menu::ActiveModel;
use crate::vo::{err_result_msg, ok_result_msg, ok_result_page};
use crate::vo::menu_vo::{*};

// 查询菜单
#[handler]
pub async fn menu_list(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<MenuListReq>().await;
    log::info!("menu_list params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let mut list_data: Vec<MenuListData> = Vec::new();

    for menu in SysMenu::find().order_by_asc(sys_menu::Column::Sort).all(conn).await.unwrap_or_default() {
        list_data.push(MenuListData {
            id: menu.id,
            sort: menu.sort,
            status_id: menu.status_id,
            parent_id: menu.parent_id,
            menu_name: menu.menu_name.clone(),
            label: menu.menu_name,
            menu_url: menu.menu_url,
            icon: menu.menu_icon.unwrap_or_default(),
            api_url: menu.api_url,
            remark: menu.remark.unwrap_or_default(),
            menu_type: menu.menu_type,
            create_time: menu.create_time.to_string(),
            update_time: menu.update_time.to_string(),
        })
    }

    res.render(Json(ok_result_page(list_data, 0)))
}

// 添加菜单
#[handler]
pub async fn menu_save(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let menu = req.parse_json::<MenuSaveReq>().await.unwrap();
    log::info!("menu_save params: {:?}", &menu);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let sys_menu = ActiveModel {
        id: NotSet,
        status_id: Set(menu.status_id),
        sort: Set(menu.sort),
        parent_id: Set(menu.parent_id),
        menu_name: Set(menu.menu_name),
        menu_url: Set(menu.menu_url.unwrap_or_default()),
        api_url: Set(menu.api_url.unwrap_or_default()),
        menu_icon: Set(menu.icon),
        remark: Set(menu.remark),
        menu_type: Set(menu.menu_type),
        ..Default::default()
    };

    SysMenu::insert(sys_menu).exec(conn).await.unwrap();
    res.render(Json(ok_result_msg("添加菜单信息成功!")))
}

// 更新菜单
#[handler]
pub async fn menu_update(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let menu = req.parse_json::<MenuUpdateReq>().await.unwrap();
    log::info!("menu_update params: {:?}", &menu);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    if SysMenu::find_by_id(menu.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        return res.render(Json(err_result_msg("菜单不存在,不能更新!")));
    }

    let sys_menu = ActiveModel {
        id: Set(menu.id),
        status_id: Set(menu.status_id),
        sort: Set(menu.sort),
        parent_id: Set(menu.parent_id),
        menu_name: Set(menu.menu_name),
        menu_url: Set(menu.menu_url.unwrap_or_default()),
        api_url: Set(menu.api_url.unwrap_or_default()),
        menu_icon: Set(menu.icon),
        remark: Set(menu.remark),
        menu_type: Set(menu.menu_type),
        ..Default::default()
    };

    SysMenu::update(sys_menu).exec(conn).await.unwrap();
    res.render(Json(ok_result_msg("更新菜单信息成功!")))
}

// 删除菜单信息
#[handler]
pub async fn menu_delete(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<MenuDeleteReq>().await.unwrap();
    log::info!("menu_delete params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    if SysMenu::find_by_id(item.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        return res.render(Json(err_result_msg("菜单不存在,不能删除!")));
    }

    if SysMenu::find().filter(sys_menu::Column::ParentId.eq(item.id.clone())).count(conn).await.unwrap_or_default() > 0 {
        return res.render(Json(err_result_msg("有下级菜单,不能直接删除!")));
    }

    SysMenu::delete_by_id(item.id.clone()).exec(conn).await.unwrap();
    res.render(Json(ok_result_msg("删除菜单信息成功!")))
}