create table if not exists users (
    id integer primary key not null,
    name varchar(64) null, -- "账号昵称"
    email varchar(64) null, -- "邮箱, 可用于登录需唯一"
    phone varchar(64) null, -- "手机号, 可用于登录需唯一"
    password varchar(64) null, -- "静态密码"
    oidc_claims_sub varchar(64) null, -- '标准 OIDC IdP 授权服务(如:Keycloak)返回的 claims sub 用于绑定唯一标识用户'
    oidc_claims_name varchar(64) null, -- '标准 OIDC IdP 授权服务(如:Keycloak)返回的 chiams name/preferer_name'
    oidc_claims_email varchar(64) null, -- '标准 OIDC IdP 授权服务(如:Keycloak)返回的 chimas email'
    github_claims_sub varchar(64) null, -- 'Github IdP 授权服务返回的 claim sub 用于绑定唯一标识用户'
    github_claims_name varchar(64) null, -- 'Github IdP 授权服务返回的 claims name/preferer_name'
    github_claims_email varchar(64) null, -- 'Github IdP 授权服务返回的 claims email'
    google_claims_sub varchar(64) null, -- 'Google IdP 授权服务返回的 sub claim 用于绑定唯一标识用户'
    google_claims_name varchar(64) null, -- 'Google IdP 授权服务返回的 claims name/preferer_name'
    google_claims_email varchar(64) null, -- 'Google IdP 授权服务返回的 claims email'
    lang varchar(64) null,
    status integer null default 0,
    create_by varchar(64) null,
    create_time integer default current_timestamp,
    update_by varchar(64) null,
    update_time integer default current_timestamp,
    del_flag integer not null default 0
);