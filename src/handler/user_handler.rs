use std::collections::HashMap;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use rbs::to_value;
use salvo::{Request, Response};
use salvo::prelude::*;
use crate::model::user::{SysUser};
use crate::model::menu::{SysMenu};
use crate::model::role::{SysRole};
use crate::model::user_role::{SysUserRole};
use crate::RB;
use crate::utils::error::WhoUnfollowedError;
use crate::vo::user_vo::*;
use crate::utils::jwt_util::JWTToken;
use crate::vo::{BaseResponse, handle_result};

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UserLoginReq>().await.unwrap();
    log::info!("user login params: {:?}", &item);


    let user_result = SysUser::select_by_mobile(&mut RB.clone(), &item.mobile).await;
    log::info!("select_by_mobile: {:?}",user_result);

    match user_result {
        Ok(u) => {
            match u {
                None => {
                    let resp = BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: Some("None"),
                    };
                    return res.render(Json(resp));
                }
                Some(user) => {
                    let id = user.id.unwrap();
                    let username = user.user_name;
                    let password = user.password;

                    if password.ne(&item.password) {
                        let resp = BaseResponse {
                            msg: "密码不正确".to_string(),
                            code: 1,
                            data: Some("None"),
                        };
                        return res.render(Json(resp));
                    }

                    let btn_menu = query_btn_menu(&id).await;

                    if btn_menu.len() == 0 {
                        let resp = BaseResponse {
                            msg: "用户没有分配角色或者菜单,不能登录".to_string(),
                            code: 1,
                            data: Some("None"),
                        };

                        return res.render(Json(resp));
                    }

                    match JWTToken::new(id, &username, btn_menu).create_token("123") {
                        Ok(token) => {
                            let resp = BaseResponse {
                                msg: "successful".to_string(),
                                code: 0,
                                data: Some(UserLoginData {
                                    mobile: item.mobile.to_string(),
                                    token,
                                }),
                            };

                            res.render(Json(resp))
                        }
                        Err(err) => {
                            let er = match err {
                                WhoUnfollowedError::JwtTokenError(s) => { s }
                                _ => "no math error".to_string()
                            };
                            let resp = BaseResponse {
                                msg: er,
                                code: 1,
                                data: Some("None"),
                            };

                            res.render(Json(resp))
                        }
                    }
                }
            }
        }

        Err(err) => {
            log::info!("select_by_column: {:?}",err);
            let resp = BaseResponse {
                msg: "查询用户异常".to_string(),
                code: 1,
                data: Some("None"),
            };
            return res.render(Json(resp));
        }
    }
}

async fn query_btn_menu(id: &i32) -> Vec<String> {
    let user_role = SysUserRole::select_by_column(&mut RB.clone(), "user_id", id.clone()).await;
    // 判断是不是超级管理员
    let mut is_admin = false;

    for x in user_role.unwrap() {
        if x.role_id == 1 {
            is_admin = true;
            break;
        }
    }

    let mut btn_menu: Vec<String> = Vec::new();
    if is_admin {
        let data = SysMenu::select_all(&mut RB.clone()).await;

        for x in data.unwrap() {
            btn_menu.push(x.api_url.unwrap_or_default());
        }
        log::info!("admin login: {:?}",id);
        btn_menu
    } else {
        let btn_menu_map: Vec<HashMap<String, String>> = RB.query_decode("select distinct u.api_url from sys_role_user t left join sys_role usr on t.role_id = usr.id left join sys_menu_role usrm on usr.id = usrm.role_id left join sys_menu u on usrm.menu_id = u.id where t.user_id = ?", vec![to_value!(id)]).await.unwrap();
        for x in btn_menu_map {
            btn_menu.push(x.get("api_url").unwrap().to_string());
        }
        log::info!("ordinary login: {:?}",id);
        btn_menu
    }
}

#[handler]
pub async fn query_user_role(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryUserRoleReq>().await.unwrap();
    log::info!("query_user_role params: {:?}", item);

    let user_role = SysUserRole::select_by_column(&mut RB.clone(), "user_id", item.user_id).await;
    let mut user_role_ids: Vec<i32> = Vec::new();

    for x in user_role.unwrap() {
        user_role_ids.push(x.role_id);
    }

    let sys_role = SysRole::select_all(&mut RB.clone()).await;

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();

    for x in sys_role.unwrap() {
        sys_role_list.push(UserRoleList {
            id: x.id.unwrap(),
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().0.to_string(),
            update_time: x.update_time.unwrap().0.to_string(),
        });
    }

    let resp = QueryUserRoleResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryUserRoleData {
            sys_role_list,
            user_role_ids,
        },
    };

    res.render(Json(resp))
}

#[handler]
pub async fn update_user_role(req: &mut Request, res: &mut Response) {
    let user_role = req.parse_json::<UpdateUserRoleReq>().await.unwrap();
    log::info!("update_user_role params: {:?}", user_role);

    let user_id = user_role.user_id.clone();
    let role_ids = &user_role.role_ids;
    let len = user_role.role_ids.len();

    if user_id.clone() == 1 {
        let resp = BaseResponse {
            msg: "不能修改超级管理员的角色".to_string(),
            code: 1,
            data: Some("None"),
        };
        return res.render(Json(resp));
    }

    let sys_result = SysUserRole::delete_by_column(&mut RB.clone(), "user_id", user_id.clone()).await;

    if sys_result.is_err() {
        let resp = BaseResponse {
            msg: "更新用户角色异常".to_string(),
            code: 1,
            data: Some("None"),
        };
        return res.render(Json(resp));
    }

    let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(SysUserRole {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    let result = SysUserRole::insert_batch(&mut RB.clone(), &sys_role_user_list, len as u64).await;

    res.render(Json(handle_result(result)))
}


#[handler]
pub async fn query_user_menu(depot: &mut Depot, res: &mut Response) {
    log::info!("query user menu params {:?}",depot);
    let user_id = depot.get::<i32>("userId").copied().unwrap();
    let username = depot.get::<String>("username").unwrap().to_string();
    log::info!("query user menu params {:?}",user_id);
    log::info!("query user menu params {:?}",username);

    //根据id查询用户
    let result = SysUser::select_by_id(&mut RB.clone(), user_id).await;

    match result {
        Ok(user) => {
            match user {
                // 用户不存在的情况
                None => {
                    res.render(Json(BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: Some(""),
                    }))
                }
                Some(u) => {
                    let data = SysMenu::select_page(&mut RB.clone(), &PageRequest::new(1, 1000)).await;

                    let mut sys_menu: Vec<MenuUserList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut btn_menu_str: String = String::new();

                    for x in data.unwrap().records {
                        let y = x.clone();
                        if y.menu_type != 3 {
                            sys_menu.push(MenuUserList {
                                id: y.id.unwrap(),
                                parent_id: y.parent_id,
                                name: y.menu_name,
                                icon: y.menu_icon.unwrap_or_default(),
                                api_url: y.api_url.as_ref().unwrap().to_string(),
                                menu_type: y.menu_type,
                                path: y.menu_url.unwrap_or_default(),
                            });
                        }

                        btn_menu.push(x.api_url.unwrap_or_default());
                        btn_menu_str.push_str(&x.menu_name);
                        btn_menu_str.push_str(&",")
                    }

                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(QueryUserMenuData {
                            sys_menu,
                            btn_menu,
                            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                            name: u.user_name,
                        }),
                    };
                    res.render(Json(resp))
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => {
            res.render(Json(BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: Some(""),
            }))
        }
    }
}


#[handler]
pub async fn user_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UserListReq>().await.unwrap();
    log::info!("query user_list params: {:?}", &item);


    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = SysUser::select_page_by_name(&mut RB.clone(), page, mobile, status_id).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut user_list_resp: Vec<UserListData> = Vec::new();

            for x in d.records {
                user_list_resp.push(UserListData {
                    id: x.id.unwrap(),
                    sort: x.sort,
                    status_id: x.status_id,
                    mobile: x.mobile,
                    user_name: x.user_name,
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.create_time.unwrap().0.to_string(),
                    update_time: x.update_time.unwrap().0.to_string(),
                })
            }

            UserListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(user_list_resp),
            }
        }
        Err(err) => {
            UserListResp {
                msg: err.to_string(),
                code: 1,
                success: true,
                total: 0,
                data: None,
            }
        }
    };

    res.render(Json(resp))
}


#[handler]
pub async fn user_save(req: &mut Request, res: &mut Response) {
    let user = req.parse_json::<UserSaveReq>().await.unwrap();
    log::info!("user_save params: {:?}", &user);

    let sys_user = SysUser {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: user.status_id,
        sort: user.sort,
        mobile: user.mobile,
        user_name: user.user_name,
        remark: user.remark,
        password: "123456".to_string(),//默认密码为123456,暂时不加密
    };

    let result = SysUser::insert(&mut RB.clone(), &sys_user).await;

    res.render(Json(handle_result(result)))
}


#[handler]
pub async fn user_update(req: &mut Request, res: &mut Response) {
    let user = req.parse_json::<UserUpdateReq>().await.unwrap();
    log::info!("user_update params: {:?}", &user);

    let result = SysUser::select_by_id(&mut RB.clone(), user.id.clone()).await.unwrap();

    match result {
        None => {
            res.render(Json(BaseResponse {
                msg: "用户不存在".to_string(),
                code: 1,
                data: Some("None".to_string()),
            }))
        }
        Some(s_user) => {
            let sys_user = SysUser {
                id: Some(user.id),
                create_time: s_user.create_time,
                update_time: Some(DateTime::now()),
                status_id: user.status_id,
                sort: user.sort,
                mobile: user.mobile,
                user_name: user.user_name,
                remark: user.remark,
                password: s_user.password,
            };

            let result = SysUser::update_by_column(&mut RB.clone(), &sys_user, "id").await;

            res.render(Json(handle_result(result)))
        }
    }
}


#[handler]
pub async fn user_delete(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UserDeleteReq>().await.unwrap();
    log::info!("user_delete params: {:?}", &item);

    let ids = item.ids;
    for id in ids {
        if id != 1 {//id为1的用户为系统预留用户,不能删除
            SysUser::delete_by_column(&mut RB.clone(), "id", &id).await
        }
    }

    res.render(Json(BaseResponse {
        msg: "successful".to_string(),
        code: 0,
        data: Some("None".to_string()),
    }))
}

#[handler]
pub async fn update_user_password(req: &mut Request, res: &mut Response) {
    let user_pwd = req.parse_json::<UpdateUserPwdReq>().await.unwrap();
    log::info!("update_user_pwd params: {:?}", &user_pwd);

    let user_result = SysUser::select_by_id(&mut RB.clone(), user_pwd.id).await;

    match user_result {
        Ok(user) => {
            let mut sys_user = user.unwrap();
            sys_user.password = user_pwd.re_pwd;
            let result = SysUser::update_by_column(&mut RB.clone(), &sys_user, "id").await;

            res.render(Json(handle_result(result)))
        }
        Err(err) => {
            let resp = BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: Some("None"),
            };
            res.render(Json(resp))
        }
    }
}