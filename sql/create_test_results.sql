DROP TABLE IF EXISTS test_results CASCADE;

CREATE TABLE test_results (
    id SERIAL PRIMARY KEY,

    -- Contexte d'exécution
    execution_date     TIMESTAMP NOT NULL,
    environment        VARCHAR(50),
    ci_tool            VARCHAR(50),
    git_branch         VARCHAR(100),
    job_name           VARCHAR(100),
    build_number       VARCHAR(50),
    hostname           VARCHAR(100),

    -- Suite & test
    test_suite_name    VARCHAR(255) NOT NULL,
    test_case_name     VARCHAR(255) NOT NULL,
    test_status        VARCHAR(20)  NOT NULL,

    -- Temps
    test_start_time    TIMESTAMP NOT NULL,
    test_end_time      TIMESTAMP NOT NULL,
    test_duration      INT,

    -- Statistiques cumulées
    total_tests        INT,
    passed_tests       INT,
    failed_tests       INT,
    skipped_tests      INT,

    -- Versions & contexte
    katalon_version    VARCHAR(50),
    browser            VARCHAR(100),
    feature            VARCHAR(100),
    test_type          VARCHAR(50),

    -- Erreurs
    error_message      TEXT
);

-- Index pour accélérer Grafana
CREATE INDEX idx_test_suite_name   ON test_results(test_suite_name);
CREATE INDEX idx_test_case_name    ON test_results(test_case_name);
CREATE INDEX idx_test_status       ON test_results(test_status);
CREATE INDEX idx_execution_date    ON test_results(execution_date);
CREATE INDEX idx_git_branch        ON test_results(git_branch);
CREATE INDEX idx_ci_tool           ON test_results(ci_tool);
