This will just be a form POST to the address http://localhost:3000/settings

1. GET http://localhost:3000/login
2. Scrape `authenticity_token`
3. POST credentials to http://localhost:3000/login
4. GET http://localhost:3000/settings
5. Scrape  `authenticity_token` from the `edit_user` `form` (there are multiple forms on the page)
6. POST matrix nick to http://localhost:3000/settings

Get the matrix nicks by visting

http://localhost:3000/u/test.json

where `test` is the username.
