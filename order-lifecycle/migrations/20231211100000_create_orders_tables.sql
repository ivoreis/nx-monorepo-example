CREATE TYPE country_enum AS ENUM ('uk');
CREATE TYPE product_enum AS ENUM ('car');
CREATE TYPE action_enum AS ENUM (
    'issue',
    'activate',
    'suspend',
    'update',
    'terminate',
    'cancel'
);
CREATE TYPE status_enum AS ENUM ('pending', 'in_progress', 'completed', 'failed');
CREATE TYPE order_state_enum AS ENUM (
    'requested',
    'issued',
    'active',
    'suspended',
    'terminated',
    'cancelled'
);

CREATE TABLE orders (
    id UUID PRIMARY KEY,
    state order_state_enum NOT NULL,
    country country_enum NOT NULL,
    product product_enum NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    order_id UUID REFERENCES orders(id) NOT NULL,
    action action_enum NOT NULL,
    status status_enum DEFAULT 'pending' NOT NULL,
    payload JSONB NOT NULL,
    idempotency_key TEXT UNIQUE,
    max_retries INT DEFAULT 0 NOT NULL,
    task_sequence INT DEFAULT 0 NOT NULL,
    scheduled_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);
CREATE INDEX idx_task_sequence ON tasks (order_id, task_sequence);