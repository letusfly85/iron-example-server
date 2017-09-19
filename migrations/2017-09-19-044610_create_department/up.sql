create table department (
  id integer not null auto_increment,
  name varchar(30) not null,
  description varchar(200),
  -- created_by_service_name varchar(256) default 'training' not null,
  -- created_by_service_user_id varchar(256) default '0' not null,
  -- created_at datetime not null default current_timestamp,
  -- updated_by_service_name varchar(256) default 'training' not null,
  -- updated_by_service_user_id varchar(256) default '0' not null,
  -- updated_at datetime not null default current_timestamp on update current_timestamp,
  primary key(id)
)
