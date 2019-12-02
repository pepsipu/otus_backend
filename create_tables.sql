CREATE TABLE images (
    id serial primary key,
    name varchar(35),
    download varchar(90),
    description varchar(650),
    secret varchar(20)
);
CREATE TABLE vulnerabilities (
    id serial primary key,
    image_id int references images(id),
    command varchar(700),
    status_code int,
    success varchar(255),
    points int
);
CREATE TABLE competitors (
    id serial primary key,
    image_id int references images(id),
    name varchar(255),
    session bytea,
    score int
);
CREATE TABLE time_slices (
    competitor int references competitors(id),
    current_points int,
    timestamp timestamptz default now()
);
CREATE TABLE updates (
    header varchar(255),
    content varchar(65535),
    timestamp timestamptz default now()
);