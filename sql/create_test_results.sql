CREATE TABLE test_results (
    id SERIAL PRIMARY KEY,
    execution_date TIMESTAMP NOT NULL,
    duration INT,
    environment VARCHAR(50),
    ci_tool VARCHAR(50),
    test_suite_name VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL,
    total_tests INT,
    passed_tests INT,
    failed_tests INT,
    skipped_tests INT,
    test_case_name VARCHAR(255) NOT NULL,
    test_status VARCHAR(20) NOT NULL,
    test_start_time TIMESTAMP NOT NULL,
    test_end_time TIMESTAMP NOT NULL,
    test_duration INT,
    error_message TEXT,
    feature VARCHAR(100),
    test_type VARCHAR(50),
    last_success_date DATE,
    consecutive_failures INT DEFAULT 0
);

CREATE INDEX idx_test_suite_name ON test_results(test_suite_name);
CREATE INDEX idx_test_case_name ON test_results(test_case_name);
CREATE INDEX idx_status ON test_results(test_status);
CREATE INDEX idx_execution_date ON test_results(execution_date);
