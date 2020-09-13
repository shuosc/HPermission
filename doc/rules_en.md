# Matching rules

In practice, the auth scenarios we are facing with can be very complex, here are several examples from our API platform:

- Semester `/api/semester?date=xxx`.
  - Everybody can read.
  - Administrators can write.
- Course `/api/course/`.
  - Registered users can read.
  - Administrators can write.
- Student `/api/student/id`.
  - Students themselves can read and write.
  - Administrators can read and write.
- Course selection url `/api/course-selection?student_id=xxx&semester_id=yyy`.
  - Students themselves can read.
- Student todo items `/api/todo?student_id=xxx&semester_id=yyy`.
  - Students themselves can read and write.

We abstract the above possibilities into:

- Users who are administrators can read/write.
- If user's id in header equals with a section in the url then the user can read/write.
  - "section" here means either a segment in url or a query string.

We choose to use a config file to specify each url and its permissions:

```toml
superuser = "17120238"
[[urls]]
prefix = "/api/semester"
read = { everybody = true } # for everybody: readable
write = { superuser = true } # for superuser: writable
[[urls]]
prefix = "/api/course"
read = { authed = true } # for authed user: readable
write = { superuser = true }
[[urls]]
prefix = "/api/student"
[urls.write.authed]
url = 2 # if the 2nd section in url (count from 0) equals to the id in jwt token in header, the user can write
[urls.read.authed]
url = 2
[[urls]]
prefix = "/api/course-selection"
write = { superuser = true }
[urls.read]
superuser = true
[urls.read.authed]
query = "student_id" # 
[[urls]]
prefix = "/api/todo"
[urls.write]
superuser = true
[urls.write.authed]
query = "student_id" # if the student_id field in query equals to the id in jwt token in header, the user can write
[urls.read]
superuser = true
[urls.read.authed]
query = "student_id"
```

Will check in order of:
1. everybody
2. authed
3. superuser