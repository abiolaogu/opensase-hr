-- OpenSASE HR Platform - Comprehensive Database Schema
-- Nigerian HR Compliance (PAYE, PenCom, NDPR)

-- ============================================================================
-- CORE TABLES
-- ============================================================================

-- Departments
CREATE TABLE IF NOT EXISTS departments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) UNIQUE,
    parent_id UUID REFERENCES departments(id),
    manager_id UUID,
    budget DECIMAL(15, 2),
    cost_center VARCHAR(100),
    location VARCHAR(255),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Positions/Job Titles
CREATE TABLE IF NOT EXISTS positions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    department_id UUID REFERENCES departments(id),
    grade_level VARCHAR(50),
    salary_band_id UUID,
    job_description TEXT,
    requirements JSONB,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Enhanced Employees Table (Nigerian focus)
CREATE TABLE IF NOT EXISTS employees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    employee_id VARCHAR(50) UNIQUE NOT NULL,
    
    -- Personal Information
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    middle_name VARCHAR(100),
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20),
    date_of_birth DATE,
    gender VARCHAR(20),
    marital_status VARCHAR(20),
    
    -- Nigerian Address
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(100),
    state_of_origin VARCHAR(50),  -- Nigerian state
    lga VARCHAR(100),              -- Local Government Area
    country VARCHAR(100) DEFAULT 'Nigeria',
    
    -- Employment Information
    department_id UUID REFERENCES departments(id),
    position_id UUID REFERENCES positions(id),
    manager_id UUID REFERENCES employees(id),
    employment_type VARCHAR(50) DEFAULT 'full_time', -- full_time, contract, intern
    hire_date DATE NOT NULL,
    confirmation_date DATE,
    termination_date DATE,
    status VARCHAR(50) DEFAULT 'active', -- active, on_leave, suspended, terminated
    
    -- Nigerian Bank Details (for payroll)
    bank_name VARCHAR(100),
    account_number VARCHAR(20),
    account_name VARCHAR(255),
    
    -- Nigerian Tax & Statutory
    tin VARCHAR(20),               -- Tax Identification Number
    pension_pin VARCHAR(20),       -- PenCom Pension PIN
    pension_pfa VARCHAR(100),      -- Pension Fund Administrator
    rsa_number VARCHAR(30),        -- Retirement Savings Account
    nhf_number VARCHAR(20),        -- National Housing Fund
    
    -- Emergency Contact
    emergency_contact_name VARCHAR(255),
    emergency_contact_phone VARCHAR(20),
    emergency_contact_relationship VARCHAR(50),
    
    -- Metadata
    profile_photo_url VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Salary Structures
CREATE TABLE IF NOT EXISTS salary_structures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    effective_date DATE NOT NULL,
    
    -- Components
    basic_salary DECIMAL(15, 2) NOT NULL,
    housing_allowance DECIMAL(15, 2) DEFAULT 0,
    transport_allowance DECIMAL(15, 2) DEFAULT 0,
    meal_allowance DECIMAL(15, 2) DEFAULT 0,
    utility_allowance DECIMAL(15, 2) DEFAULT 0,
    other_allowances JSONB DEFAULT '{}',
    
    -- Nigerian deductions applicable
    paye_applicable BOOLEAN DEFAULT true,
    pension_applicable BOOLEAN DEFAULT true,
    nhf_applicable BOOLEAN DEFAULT true,
    
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Employee Salary Assignment
CREATE TABLE IF NOT EXISTS employee_salaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id),
    salary_structure_id UUID REFERENCES salary_structures(id),
    
    -- Override amounts (if different from structure)
    basic_salary DECIMAL(15, 2) NOT NULL,
    housing_allowance DECIMAL(15, 2) DEFAULT 0,
    transport_allowance DECIMAL(15, 2) DEFAULT 0,
    other_allowances JSONB DEFAULT '{}',
    
    effective_from DATE NOT NULL,
    effective_to DATE,
    is_current BOOLEAN DEFAULT true,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- PAYROLL TABLES
-- ============================================================================

-- Payroll Runs
CREATE TABLE IF NOT EXISTS payroll_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    run_date TIMESTAMPTZ,
    
    status VARCHAR(50) DEFAULT 'draft', -- draft, processing, pending_approval, approved, paid
    
    -- Totals
    total_employees INTEGER DEFAULT 0,
    total_gross DECIMAL(15, 2) DEFAULT 0,
    total_deductions DECIMAL(15, 2) DEFAULT 0,
    total_net DECIMAL(15, 2) DEFAULT 0,
    total_employer_contributions DECIMAL(15, 2) DEFAULT 0,
    
    -- Approval
    processed_by UUID REFERENCES employees(id),
    processed_at TIMESTAMPTZ,
    approved_by UUID REFERENCES employees(id),
    approved_at TIMESTAMPTZ,
    
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Payroll Items (Individual Payslips)
CREATE TABLE IF NOT EXISTS payroll_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payroll_run_id UUID NOT NULL REFERENCES payroll_runs(id),
    employee_id UUID NOT NULL REFERENCES employees(id),
    
    -- Earnings
    basic_salary DECIMAL(15, 2) NOT NULL,
    housing_allowance DECIMAL(15, 2) DEFAULT 0,
    transport_allowance DECIMAL(15, 2) DEFAULT 0,
    meal_allowance DECIMAL(15, 2) DEFAULT 0,
    utility_allowance DECIMAL(15, 2) DEFAULT 0,
    other_allowances JSONB DEFAULT '{}',
    gross_pay DECIMAL(15, 2) NOT NULL,
    
    -- Nigerian Statutory Deductions
    paye_tax DECIMAL(15, 2) DEFAULT 0,
    pension_employee DECIMAL(15, 2) DEFAULT 0,  -- 8%
    pension_employer DECIMAL(15, 2) DEFAULT 0,  -- 10%
    nhf_deduction DECIMAL(15, 2) DEFAULT 0,     -- 2.5% of basic
    
    -- Other Deductions
    loan_repayment DECIMAL(15, 2) DEFAULT 0,
    other_deductions JSONB DEFAULT '{}',
    total_deductions DECIMAL(15, 2) DEFAULT 0,
    
    -- Net Pay
    net_pay DECIMAL(15, 2) NOT NULL,
    
    -- Bank Details (snapshot)
    bank_name VARCHAR(100),
    account_number VARCHAR(20),
    account_name VARCHAR(255),
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- LEAVE MANAGEMENT TABLES
-- ============================================================================

-- Leave Types
CREATE TABLE IF NOT EXISTS leave_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(100) NOT NULL,           -- annual, sick, maternity, paternity, compassionate, study
    code VARCHAR(20) UNIQUE NOT NULL,
    default_days INTEGER NOT NULL,
    is_paid BOOLEAN DEFAULT true,
    requires_approval BOOLEAN DEFAULT true,
    requires_document BOOLEAN DEFAULT false,  -- e.g., sick leave > 3 days needs medical cert
    document_threshold_days INTEGER DEFAULT 3,
    max_carry_over INTEGER DEFAULT 5,
    gender_restriction VARCHAR(10),        -- 'male', 'female', or null for all
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Leave Balances
CREATE TABLE IF NOT EXISTS leave_balances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id),
    leave_type_id UUID NOT NULL REFERENCES leave_types(id),
    year INTEGER NOT NULL,
    
    entitled_days DECIMAL(5, 2) NOT NULL,
    used_days DECIMAL(5, 2) DEFAULT 0,
    pending_days DECIMAL(5, 2) DEFAULT 0,
    carried_over DECIMAL(5, 2) DEFAULT 0,
    
    UNIQUE(employee_id, leave_type_id, year),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Leave Requests
CREATE TABLE IF NOT EXISTS leave_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id),
    leave_type_id UUID NOT NULL REFERENCES leave_types(id),
    
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    days_requested DECIMAL(5, 2) NOT NULL,
    half_day BOOLEAN DEFAULT false,
    
    reason TEXT,
    document_url VARCHAR(500),
    
    relief_officer_id UUID REFERENCES employees(id),
    handover_notes TEXT,
    
    status VARCHAR(50) DEFAULT 'pending', -- pending, approved, rejected, cancelled
    approved_by UUID REFERENCES employees(id),
    approved_at TIMESTAMPTZ,
    rejection_reason TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Nigerian Public Holidays
CREATE TABLE IF NOT EXISTS public_holidays (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,  -- null for system-wide Nigerian holidays
    name VARCHAR(255) NOT NULL,
    date DATE NOT NULL,
    is_recurring BOOLEAN DEFAULT false,
    recurrence_month INTEGER,
    recurrence_day INTEGER,
    year INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- PERFORMANCE MANAGEMENT TABLES
-- ============================================================================

-- Performance Cycles
CREATE TABLE IF NOT EXISTS performance_cycles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    cycle_type VARCHAR(50) DEFAULT 'annual', -- annual, quarterly, probation
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    
    -- Weights
    goals_weight DECIMAL(3, 2) DEFAULT 0.70,
    competencies_weight DECIMAL(3, 2) DEFAULT 0.30,
    
    status VARCHAR(50) DEFAULT 'draft', -- draft, active, closed
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Performance Reviews
CREATE TABLE IF NOT EXISTS performance_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cycle_id UUID NOT NULL REFERENCES performance_cycles(id),
    employee_id UUID NOT NULL REFERENCES employees(id),
    reviewer_id UUID REFERENCES employees(id),
    
    -- Ratings (1-5 scale)
    self_rating DECIMAL(3, 2),
    manager_rating DECIMAL(3, 2),
    final_rating DECIMAL(3, 2),
    
    -- Goals and Competencies (JSONB for flexibility)
    goals JSONB DEFAULT '[]',
    competencies JSONB DEFAULT '[]',
    
    self_review_submitted_at TIMESTAMPTZ,
    manager_review_submitted_at TIMESTAMPTZ,
    
    status VARCHAR(50) DEFAULT 'pending', -- pending, self_submitted, manager_submitted, completed
    self_comments TEXT,
    manager_comments TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Goals
CREATE TABLE IF NOT EXISTS goals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id),
    cycle_id UUID REFERENCES performance_cycles(id),
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(50) DEFAULT 'individual', -- individual, team, company
    
    -- SMART criteria
    specific TEXT,
    measurable TEXT,
    achievable TEXT,
    relevant TEXT,
    time_bound DATE,
    
    weight DECIMAL(3, 2) DEFAULT 0.20,  -- % of total goals score
    target_value DECIMAL(10, 2),
    current_value DECIMAL(10, 2) DEFAULT 0,
    progress_percentage INTEGER DEFAULT 0,
    
    status VARCHAR(50) DEFAULT 'active', -- active, completed, cancelled
    rating DECIMAL(3, 2),
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- RECRUITMENT TABLES
-- ============================================================================

-- Job Postings
CREATE TABLE IF NOT EXISTS job_postings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    department_id UUID REFERENCES departments(id),
    position_id UUID REFERENCES positions(id),
    
    description TEXT NOT NULL,
    requirements JSONB,              -- skills, experience, education
    responsibilities JSONB,
    
    salary_min DECIMAL(15, 2),
    salary_max DECIMAL(15, 2),
    show_salary BOOLEAN DEFAULT false,
    
    location VARCHAR(255),
    employment_type VARCHAR(50) DEFAULT 'full_time',
    experience_level VARCHAR(50),    -- entry, mid, senior, lead
    
    status VARCHAR(50) DEFAULT 'draft', -- draft, published, closed, filled
    posted_date TIMESTAMPTZ,
    closing_date DATE,
    
    vacancies INTEGER DEFAULT 1,
    applications_count INTEGER DEFAULT 0,
    
    created_by UUID REFERENCES employees(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Job Applications
CREATE TABLE IF NOT EXISTS job_applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_posting_id UUID NOT NULL REFERENCES job_postings(id),
    
    -- Applicant Info
    applicant_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    
    cv_url VARCHAR(500),
    cover_letter TEXT,
    linkedin_url VARCHAR(500),
    
    -- AI Scoring
    ai_score DECIMAL(5, 2),
    ai_analysis JSONB,               -- skills_matched, skills_missing, summary
    
    -- Pipeline Stage
    stage VARCHAR(50) DEFAULT 'received', -- received, screening, interview, offer, hired, rejected
    stage_history JSONB DEFAULT '[]',
    
    -- Interview
    interview_scheduled_at TIMESTAMPTZ,
    interview_notes TEXT,
    interview_rating DECIMAL(3, 2),
    
    -- Outcome
    rejection_reason TEXT,
    offer_salary DECIMAL(15, 2),
    offer_sent_at TIMESTAMPTZ,
    offer_accepted_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- BENEFITS TABLES
-- ============================================================================

-- Benefit Plans
CREATE TABLE IF NOT EXISTS benefit_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,       -- hmo, life_insurance, pension_avc, allowance
    provider VARCHAR(255),
    
    coverage_details JSONB,
    cost_employee DECIMAL(15, 2) DEFAULT 0,
    cost_employer DECIMAL(15, 2) DEFAULT 0,
    
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Employee Benefits Enrollment
CREATE TABLE IF NOT EXISTS employee_benefits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id),
    benefit_plan_id UUID NOT NULL REFERENCES benefit_plans(id),
    
    enrolled_date DATE NOT NULL,
    status VARCHAR(50) DEFAULT 'active', -- active, cancelled, expired
    
    dependents JSONB DEFAULT '[]',    -- list of covered dependents
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Benefit Claims
CREATE TABLE IF NOT EXISTS benefit_claims (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id),
    benefit_plan_id UUID NOT NULL REFERENCES benefit_plans(id),
    
    claim_type VARCHAR(100) NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    description TEXT,
    receipt_url VARCHAR(500),
    
    status VARCHAR(50) DEFAULT 'pending', -- pending, approved, rejected, paid
    approved_by UUID REFERENCES employees(id),
    approved_at TIMESTAMPTZ,
    rejection_reason TEXT,
    paid_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- COMPLIANCE & AUDIT TABLES
-- ============================================================================

-- Audit Logs (Immutable)
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    entity_type VARCHAR(100) NOT NULL,  -- 'employee', 'payroll_run', etc.
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,         -- 'create', 'update', 'delete', 'view'
    
    actor_id UUID,
    actor_type VARCHAR(50),              -- 'user', 'system', 'api'
    
    changes JSONB,                       -- before/after values
    metadata JSONB,                      -- additional context
    
    ip_address INET,
    user_agent TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- NDPR Data Subject Requests
CREATE TABLE IF NOT EXISTS data_subject_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    request_type VARCHAR(50) NOT NULL,   -- access, rectification, erasure, portability
    subject_email VARCHAR(255) NOT NULL,
    subject_name VARCHAR(255),
    
    description TEXT,
    
    status VARCHAR(50) DEFAULT 'pending', -- pending, processing, completed, rejected
    processed_by UUID REFERENCES employees(id),
    processed_at TIMESTAMPTZ,
    response TEXT,
    
    due_date DATE,  -- 30 days per NDPR
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================================
-- INDEXES
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_employees_tenant ON employees(tenant_id);
CREATE INDEX IF NOT EXISTS idx_employees_department ON employees(department_id);
CREATE INDEX IF NOT EXISTS idx_employees_manager ON employees(manager_id);
CREATE INDEX IF NOT EXISTS idx_employees_status ON employees(status);
CREATE INDEX IF NOT EXISTS idx_employees_email ON employees(email);

CREATE INDEX IF NOT EXISTS idx_payroll_runs_tenant ON payroll_runs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_payroll_runs_period ON payroll_runs(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_payroll_items_run ON payroll_items(payroll_run_id);
CREATE INDEX IF NOT EXISTS idx_payroll_items_employee ON payroll_items(employee_id);

CREATE INDEX IF NOT EXISTS idx_leave_requests_employee ON leave_requests(employee_id);
CREATE INDEX IF NOT EXISTS idx_leave_requests_status ON leave_requests(status);
CREATE INDEX IF NOT EXISTS idx_leave_requests_dates ON leave_requests(start_date, end_date);

CREATE INDEX IF NOT EXISTS idx_performance_reviews_employee ON performance_reviews(employee_id);
CREATE INDEX IF NOT EXISTS idx_performance_reviews_cycle ON performance_reviews(cycle_id);

CREATE INDEX IF NOT EXISTS idx_job_applications_posting ON job_applications(job_posting_id);
CREATE INDEX IF NOT EXISTS idx_job_applications_stage ON job_applications(stage);

CREATE INDEX IF NOT EXISTS idx_audit_logs_tenant ON audit_logs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created ON audit_logs(created_at);

-- ============================================================================
-- SEED DATA: Nigerian Public Holidays 2024-2025
-- ============================================================================

INSERT INTO public_holidays (name, date, year) VALUES
-- 2024
('New Year''s Day', '2024-01-01', 2024),
('Workers'' Day', '2024-05-01', 2024),
('Democracy Day', '2024-06-12', 2024),
('Independence Day', '2024-10-01', 2024),
('Christmas Day', '2024-12-25', 2024),
('Boxing Day', '2024-12-26', 2024),
('Good Friday', '2024-03-29', 2024),
('Easter Monday', '2024-04-01', 2024),
('Eid-el-Fitr', '2024-04-10', 2024),
('Eid-el-Fitr Day 2', '2024-04-11', 2024),
('Eid-el-Kabir', '2024-06-17', 2024),
('Eid-el-Kabir Day 2', '2024-06-18', 2024),
('Eid-el-Maulud', '2024-09-16', 2024),
-- 2025
('New Year''s Day', '2025-01-01', 2025),
('Workers'' Day', '2025-05-01', 2025),
('Democracy Day', '2025-06-12', 2025),
('Independence Day', '2025-10-01', 2025),
('Christmas Day', '2025-12-25', 2025),
('Boxing Day', '2025-12-26', 2025),
('Good Friday', '2025-04-18', 2025),
('Easter Monday', '2025-04-21', 2025)
ON CONFLICT DO NOTHING;

-- ============================================================================
-- SEED DATA: Default Leave Types (Nigerian Standard)
-- ============================================================================

INSERT INTO leave_types (tenant_id, name, code, default_days, is_paid, requires_document, document_threshold_days, max_carry_over, gender_restriction) VALUES
('00000000-0000-0000-0000-000000000000', 'Annual Leave', 'annual', 21, true, false, 0, 5, null),
('00000000-0000-0000-0000-000000000000', 'Sick Leave', 'sick', 12, true, true, 3, 0, null),
('00000000-0000-0000-0000-000000000000', 'Maternity Leave', 'maternity', 84, true, true, 0, 0, 'female'),
('00000000-0000-0000-0000-000000000000', 'Paternity Leave', 'paternity', 10, true, true, 0, 0, 'male'),
('00000000-0000-0000-0000-000000000000', 'Compassionate Leave', 'compassionate', 5, true, false, 0, 0, null),
('00000000-0000-0000-0000-000000000000', 'Study Leave', 'study', 20, false, true, 0, 0, null),
('00000000-0000-0000-0000-000000000000', 'Leave Without Pay', 'lwop', 30, false, false, 0, 0, null)
ON CONFLICT DO NOTHING;
