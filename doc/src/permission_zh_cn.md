## 权限Api

### 系统所有权限 (下拉菜单)
get api/permission/list

### 系统所有角色 (独立菜单-页面)
get api/role/list

### 角色的所有权限
post api/role/permission_detail 

### 角色的所有用户
post api/role/users 

### 用户的所有角色
post api/user/roles 

### 用户的所有权限
post api/user/permissions 

### 用户增加角色
post api/user/append_role

### 用户删除角色
post api/user/delete_role

### 用户增加权限
post api/user/append_permission

### 用户删除权限
post api/user/delete_permission