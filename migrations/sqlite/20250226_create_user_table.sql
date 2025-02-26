-- Create users table
create table if not exists users
(
	id integer primary key not null,
	username text not null unique,
	email text not null unique,
	created_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


