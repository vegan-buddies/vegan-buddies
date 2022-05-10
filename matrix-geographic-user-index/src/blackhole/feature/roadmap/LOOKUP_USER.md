Look up users
---------------

LOOKUP - matrix bot command

Commands are sent in yaml format

```
LOOKUP:
 user_type: mentee|mentor
 healpix_regions: [where, to, look]
 page_no: xxx
```

Anyone can look up a user sending a list of healpix regions. They will get a paginated list of users back.

Return yaml list of users in format

```
page: 0
no_pages: 3
- matrix_nick: @example_nick
  lobsters_address: https://veganlobsters.org/....
  healpix_region: xxx
  rating: xxx
```
