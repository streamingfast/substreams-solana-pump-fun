-- public.buy definition

-- Drop table

-- DROP TABLE public.buy;

CREATE TABLE public.buy (
	block_number int4 NOT NULL,
	amount numeric NULL,
	max_sol_cost numeric NULL
);


-- public.completeevent definition

-- Drop table

-- DROP TABLE public.completeevent;

CREATE TABLE public.completeevent (
	block_number int4 NOT NULL,
	"user" varchar(255) NULL,
	mint varchar(255) NULL,
	bonding_curve varchar(255) NULL,
	"timestamp" int8 NULL
);


-- public."create" definition

-- Drop table

-- DROP TABLE public."create";

CREATE TABLE public."create" (
	block_number int4 NOT NULL,
	"name" varchar(255) NULL,
	symbol varchar(255) NULL,
	uri varchar(255) NULL
);


-- public.createevent definition

-- Drop table

-- DROP TABLE public.createevent;

CREATE TABLE public.createevent (
	block_number int4 NOT NULL,
	"name" varchar(255) NULL,
	symbol varchar(255) NULL,
	uri varchar(255) NULL,
	mint varchar(255) NULL,
	bonding_curve varchar(255) NULL,
	"user" varchar(255) NULL
);


-- public."data" definition

-- Drop table

-- DROP TABLE public."data";

CREATE TABLE public."data" (
	block_number int4 NOT NULL
);


-- public.sell definition

-- Drop table

-- DROP TABLE public.sell;

CREATE TABLE public.sell (
	block_number int4 NOT NULL,
	amount numeric NULL,
	min_sol_output numeric NULL
);


-- public.setparams definition

-- Drop table

-- DROP TABLE public.setparams;

CREATE TABLE public.setparams (
	block_number int4 NOT NULL,
	fee_recipient varchar(255) NULL,
	initial_virtual_token_reserves numeric NULL,
	initial_virtual_sol_reserves numeric NULL,
	initial_real_token_reserves numeric NULL,
	token_total_supply numeric NULL,
	fee_basis_points numeric NULL
);


-- public.setparamsevent definition

-- Drop table

-- DROP TABLE public.setparamsevent;

CREATE TABLE public.setparamsevent (
	block_number int4 NOT NULL,
	fee_recipient varchar(255) NULL,
	initial_virtual_token_reserves numeric NULL,
	initial_virtual_sol_reserves numeric NULL,
	initial_real_token_reserves numeric NULL,
	token_total_supply numeric NULL,
	fee_basis_points numeric NULL
);


-- public.tradeevent definition

-- Drop table

-- DROP TABLE public.tradeevent;

CREATE TABLE public.tradeevent (
	block_number int4 NOT NULL,
	mint varchar(255) NULL,
	sol_amount numeric NULL,
	token_amount numeric NULL,
	is_buy bool NULL,
	"user" varchar(255) NULL,
	"timestamp" int8 NULL,
	virtual_sol_reserves numeric NULL,
	virtual_token_reserves numeric NULL,
	real_sol_reserves numeric NULL,
	real_token_reserves numeric NULL
);