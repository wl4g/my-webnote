1，请基于如下结构生成rust+axum+clap的web api服务源码
src/
├── main.rs
├── lib.rs
├── config.rs
├── routes/
│   ├── mod.rs
│   ├── users.rs
│   ├── documents.rs
│   ├── folders.rs
│   └── permission.rs
├── handlers/
│   ├── mod.rs
│   ├── users.rs
│   ├── documents.rs
│   ├── folders.rs
│   └── permission.rs
├── models/
│   ├── mod.rs
│   ├── users.rs
│   ├── documents.rs
│   ├── folders.rs
│   └── permission.rs
├── db/
│   ├── mod.rs
│   ├── mongo_users.rs
│   ├── mongo_documents.rs
│   ├── mongo_folders.rs
│   ├── mongo_permissions.rs
│   ├── sqllite_users.rs
│   ├── sqllite_documents.rs
│   ├── sqllite_folders.rs
│   └── sqllite_permission.rs
├── errors.rs
└── utils/
    ├── mod.rs
    └── helpers.rs

2，其中模块及实体结构定义如下：
sys模块下有 user、permission、settings 增删改查
biz模块下有 folders/documents 增删改查
sqllite/mongo表结构
users(id, name, phone, email, password, salt, social_auth(type:(github|oidc/keycloak),app_id,app_secret), status(0/1:enable/disabled), create_by, create_time, update_by, update_time, del_flag(0/1:normal/deleted))

permission(id, name(查看/编辑), identity(default:view/default:edit), status(0/1:enable/disabled), create_by, create_time, update_by, update_time, del_flag(0/1:normal/deleted))

settings(id, key, value, type(1/2:system/user), status(0/1:enable/disabled), create_by, create_time, update_by, update_time, del_flag(0/1:normal/deleted))

folders(id, parent_id, user_id, permissions, name, status(0/1:enable/disabled), create_by, create_time, update_by, update_time, del_flag(0/1:normal/deleted))

documents(id, user_id, folder_id, name, type(1/2:board/note), status(0/1:enable/disabled), create_by, create_time, update_by, update_time, del_flag(0/1:normal/deleted))

3，其中存储要求支持 mongo和sqlite（默认内置）存储，根据yaml配置文件通过多态设计模式实现，其中 config.rs 解析yaml结构大致如下：
server:
  bind: "0.0.0.0:8887"
  thread-max-pool: 32
  cors:
    hosts: ['']
    headers: ['']
    methods: ['*']
  auths:
    oidc:
      endpoint: "https://keycloak.example.com/realms/master/.well-known/openid-configuration"
      app-id:
      app_secret:
     github:
      endpoint: 
      app-id: 
      app_secret:

logging:
  file: /tmp/excalidraw-revezone/server.log
  pattern: "xxx" # e.g: "[yyyy-MM-dd T HH:mm:ss.SSS] [INFO] - users.rs:34 : Add to user jack01 ..."

service:
  db:
    type: sqlite # mongo|sqlite
    sqlite:
      dir: /tmp/excalidraw-revezone/
    mongo:
      url: xxxx

请按照如上3点的结构设计完整生成rust代码，请不要偷懒，不要生成不完整！