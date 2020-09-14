# 匹配规则

在实际使用的时候，我们面对的鉴权场景可能是非常复杂的，下面是我们的 API 平台上的一些例子：

- 学期情况 url `/api/semester?date=xxx`
  - 任何人可以读
  - 管理员可写
- 课程情况 url `/api/course/*`
  - 注册用户可读
  - 管理员可写
- 学生信息 `/api/student/id`
  - 学生本人可读写
  - 管理员可读写
- 学生选课 url `/api/course-selection?student_id=xxx&semester_id=yyy`
  - 学生本人可读
- 学生 todo `/api/todo?student_id=xxx&semester_id=yyy`
  - 学生本人可读写

我们将上述可能性抽象为：

- 用户为管理员则可以读/写
- 用户 header 中的身份信息和url中的某个部分一致，则可读/写
  - 某个部分可能是 url 中的某一段，也可能是某个 query string

我们选择使用配置文件指定每个 url 及其权限：

```toml
superuser = "17120238"
[[urls]]
prefix = "/api/semester"
read = { everybody = true } # 对每个人：可读
write = { superuser = true } # 对管理员：可写
[[urls]]
prefix = "/api/course"
read = { authed = true } # 对所有合法登录用户可读
write = { superuser = true }
[[urls]]
prefix = "/api/student"
[urls.write.authed]
url_segment = 2 # 将 url 的第2段（从0开始计数）和 header 中的 jwt token 中的 id 比较，相等则允许访问
[urls.read.authed]
url_segment = 2
[[urls]]
prefix = "/api/course-selection"
write = { superuser = true }
[urls.read]
superuser = true
[urls.read.authed]
query = "student_id" # 将 query 中的 student_id 和 header 中的 jwt token 中的 id 比较，相等则允许访问 
[[urls]]
prefix = "/api/todo"
[urls.write]
superuser = true
[urls.write.authed]
query = "student_id" # 将 query 中的 student_id 和 header 中的 jwt token 中的 id 比较，相等则允许访问
[urls.read]
superuser = true
[urls.read.authed]
query = "student_id"
```

检查顺序为：
1. everybody
2. authed
3. superuser
