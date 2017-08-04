CREATE table person (
	uuid varchar(36), primary key(uuid), 
	namn varchar(512) not null,
	pers_nr varchar(16),
	mail varchar(1024),
	phone varchar(32),
	street varchar(1024),
	post_nr_city varchar(1024)
)
