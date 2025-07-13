-- Your SQL goes here
CREATE TABLE "payments"(
   "id" UUID NOT NULL PRIMARY KEY,
   "correlationid" UUID NOT NULL,
   "requestedat" TIMESTAMPTZ NOT NULL,
   "amount" VARCHAR NOT NULL
);

INSERT INTO payments (id, correlationid, requestedat, amount) VALUES
    ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11','a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2023-01-15 08:30:45+00', '125.50');

INSERT INTO payments (id, correlationid, requestedat, amount) VALUES
    ('b1fffc99-9c0b-4ef8-bb6d-6bb9bd380a12','b1fffc99-9c0b-4ef8-bb6d-6bb9bd380a12', '2023-01-16 09:15:22+00', '89.99');

INSERT INTO payments (id, correlationid, requestedat, amount) VALUES
    ('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a13','c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', '2023-01-17 10:05:33+00', '220.00');