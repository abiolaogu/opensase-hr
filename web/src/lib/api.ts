/**
 * API Client for OpenSASE HR Backend
 */

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export interface ApiResponse<T> {
    success: boolean;
    data?: T;
    error?: string;
}

// Generic fetch wrapper
async function fetchApi<T>(
    endpoint: string,
    options?: RequestInit
): Promise<ApiResponse<T>> {
    try {
        const response = await fetch(`${API_BASE}${endpoint}`, {
            ...options,
            headers: {
                'Content-Type': 'application/json',
                ...options?.headers,
            },
        });

        const data = await response.json();
        return data;
    } catch (error) {
        return {
            success: false,
            error: error instanceof Error ? error.message : 'Unknown error',
        };
    }
}

// Tax calculation
export interface TaxPreviewRequest {
    monthly_gross: number;
}

export interface TaxPreview {
    gross_monthly: string;
    gross_annual: string;
    paye_monthly: string;
    paye_annual: string;
    pension_employee: string;
    pension_employer: string;
    nhf: string;
    total_deductions: string;
    net_monthly: string;
    effective_tax_rate: string;
}

export async function calculateTaxPreview(
    request: TaxPreviewRequest
): Promise<ApiResponse<TaxPreview>> {
    return fetchApi<TaxPreview>('/api/v1/payroll/tax/calculate', {
        method: 'POST',
        body: JSON.stringify(request),
    });
}

// Health check
export interface HealthResponse {
    status: string;
    version: string;
    modules: string[];
}

export async function getHealth(): Promise<ApiResponse<HealthResponse>> {
    return fetchApi<HealthResponse>('/health');
}
