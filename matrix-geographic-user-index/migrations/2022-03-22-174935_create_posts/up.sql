CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  matrix_nick VARCHAR NOT NULL,
  lobsters_address VARCHAR NOT NULL,
  healpix_region BIGINT NOT NULL
)
