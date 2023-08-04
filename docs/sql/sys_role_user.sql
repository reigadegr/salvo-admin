create table sys_role_user
(
    id           bigint  auto_increment comment '主键'
        primary key,
    create_time    datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time    datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    status_id    tinyint  default 1               not null comment '状态(1:正常，0:禁用)',
    sort         int      default 1               not null comment '排序',
    role_id      bigint                             not null comment '角色ID',
    user_id      bigint   default '0'               not null comment '用户ID'
)
    comment '角色用户关联表';

INSERT INTO sys_role_user (id, create_time, update_time, status_id, sort, role_id, user_id) VALUES (1, '2022-07-15 14:13:46', '2022-07-15 14:13:46', 1, 1, 3, 2);
INSERT INTO sys_role_user (id, create_time, update_time, status_id, sort, role_id, user_id) VALUES (2, '2022-07-30 16:51:55', '2022-07-30 16:51:55', 1, 1, 3, 12);
INSERT INTO sys_role_user (id, create_time, update_time, status_id, sort, role_id, user_id) VALUES (3, '2022-07-30 17:03:55', '2022-07-30 17:03:55', 1, 1, 4, 13);
INSERT INTO sys_role_user (id, create_time, update_time, status_id, sort, role_id, user_id) VALUES (4, '2022-07-30 17:03:55', '2022-07-30 17:03:55', 1, 1, 3, 3);
INSERT INTO sys_role_user (id, create_time, update_time, status_id, sort, role_id, user_id) VALUES (5, '2022-07-30 17:03:55', '2022-07-30 17:03:55', 1, 1, 1, 1);
