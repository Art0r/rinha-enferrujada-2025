CREATE TABLE "payments"(
   "id" BIGSERIAL PRIMARY KEY,
   "correlationid" UUID NOT NULL,
   "requestedat" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   "amount" VARCHAR NOT NULL
);
