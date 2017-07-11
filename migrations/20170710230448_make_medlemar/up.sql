CREATE table medlem (
	uuid varchar(36), primary key(uuid), 
	namn varchar(512) not null,
	persNr varchar(16),
	mail varchar(1024),
	phone varchar(32),
	street varchar(1024),
	postNrCity varchar(1024)
)
