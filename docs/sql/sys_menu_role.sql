create table sys_menu_role
(
    id           bigint  auto_increment comment '主键'
        primary key,
    create_time    datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time    datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    status_id    tinyint  default 1               not null comment '状态(1:正常，0:禁用)',
    sort         int      default 1               not null comment '排序',
    menu_id      bigint                             not null comment '菜单ID',
    role_id      bigint                             not null comment '角色ID'
)
    comment '菜单角色关联表';

INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857203, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 1, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857204, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 3, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857205, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 6, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857206, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 7, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857207, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 8, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857208, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 9, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857209, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 10, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857210, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 11, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857211, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 12, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857212, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 13, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857213, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 14, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857214, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 15, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857215, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 16, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857216, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 31, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857217, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 30, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857218, '2022-07-22 17:48:01', '2022-07-22 17:48:01', 1, 1, 2, 3);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857219, '2022-07-26 15:13:29', '2022-07-26 15:13:29', 1, 1, 32, 5);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857220, '2022-07-26 15:13:29', '2022-07-26 15:13:29', 1, 1, 1, 5);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857221, '2022-07-26 23:04:02', '2022-07-26 23:04:02', 1, 1, 1, 4);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857222, '2022-07-26 23:04:02', '2022-07-26 23:04:02', 1, 1, 30, 4);
INSERT INTO sys_menu_role (id, create_time, update_time, status_id, sort, menu_id, role_id) VALUES (1544492801968857223, '2022-07-26 23:04:02', '2022-07-26 23:04:02', 1, 1, 32, 4);
