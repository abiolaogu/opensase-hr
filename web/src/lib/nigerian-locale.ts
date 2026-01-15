/**
 * Nigerian Locale Utilities
 * 
 * Provides Nigerian-specific formatting for:
 * - Currency (₦)
 * - Dates (DD/MM/YYYY)
 * - Phone numbers (+234)
 * - Banks
 * - States and LGAs
 */

// Currency formatting with Naira symbol
export function formatNaira(amount: number | string, showDecimal = true): string {
    const num = typeof amount === 'string' ? parseFloat(amount) : amount;
    if (isNaN(num)) return '₦0.00';

    return new Intl.NumberFormat('en-NG', {
        style: 'currency',
        currency: 'NGN',
        minimumFractionDigits: showDecimal ? 2 : 0,
        maximumFractionDigits: showDecimal ? 2 : 0,
    }).format(num);
}

// Short format for large numbers (e.g., ₦1.5M)
export function formatNairaShort(amount: number): string {
    if (amount >= 1_000_000_000) {
        return `₦${(amount / 1_000_000_000).toFixed(1)}B`;
    }
    if (amount >= 1_000_000) {
        return `₦${(amount / 1_000_000).toFixed(1)}M`;
    }
    if (amount >= 1_000) {
        return `₦${(amount / 1_000).toFixed(1)}K`;
    }
    return formatNaira(amount, false);
}

// Nigerian date format (DD/MM/YYYY)
export function formatDate(date: Date | string): string {
    const d = typeof date === 'string' ? new Date(date) : date;
    return new Intl.DateTimeFormat('en-GB', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
    }).format(d);
}

// Full date format (15 January 2024)
export function formatDateLong(date: Date | string): string {
    const d = typeof date === 'string' ? new Date(date) : date;
    return new Intl.DateTimeFormat('en-NG', {
        day: 'numeric',
        month: 'long',
        year: 'numeric',
    }).format(d);
}

// Format Nigerian phone number
export function formatPhoneNumber(phone: string): string {
    // Remove all non-digits
    const digits = phone.replace(/\D/g, '');

    // Handle different formats
    if (digits.startsWith('234')) {
        return `+${digits.slice(0, 3)} ${digits.slice(3, 6)} ${digits.slice(6, 9)} ${digits.slice(9)}`;
    }
    if (digits.startsWith('0')) {
        return `+234 ${digits.slice(1, 4)} ${digits.slice(4, 7)} ${digits.slice(7)}`;
    }
    return phone;
}

// Nigerian Banks
export const NIGERIAN_BANKS = [
    { code: 'FBN', name: 'First Bank of Nigeria' },
    { code: 'ZEN', name: 'Zenith Bank' },
    { code: 'GTB', name: 'Guaranty Trust Bank' },
    { code: 'ACC', name: 'Access Bank' },
    { code: 'UBA', name: 'United Bank for Africa' },
    { code: 'FID', name: 'Fidelity Bank' },
    { code: 'STB', name: 'Stanbic IBTC Bank' },
    { code: 'UNI', name: 'Union Bank' },
    { code: 'FCMB', name: 'First City Monument Bank' },
    { code: 'STL', name: 'Sterling Bank' },
    { code: 'WEM', name: 'Wema Bank' },
    { code: 'ECO', name: 'Ecobank Nigeria' },
    { code: 'KEY', name: 'Keystone Bank' },
    { code: 'POL', name: 'Polaris Bank' },
    { code: 'KUD', name: 'Kuda Bank' },
    { code: 'OPA', name: 'OPay' },
    { code: 'MOP', name: 'Moniepoint' },
    { code: 'PAL', name: 'PalmPay' },
] as const;

// Nigerian States
export const NIGERIAN_STATES = [
    'Abia', 'Adamawa', 'Akwa Ibom', 'Anambra', 'Bauchi', 'Bayelsa',
    'Benue', 'Borno', 'Cross River', 'Delta', 'Ebonyi', 'Edo',
    'Ekiti', 'Enugu', 'FCT', 'Gombe', 'Imo', 'Jigawa',
    'Kaduna', 'Kano', 'Katsina', 'Kebbi', 'Kogi', 'Kwara',
    'Lagos', 'Nasarawa', 'Niger', 'Ogun', 'Ondo', 'Osun',
    'Oyo', 'Plateau', 'Rivers', 'Sokoto', 'Taraba', 'Yobe', 'Zamfara'
] as const;

// Get percentage change with arrow
export function formatPercentChange(value: number): { text: string; isPositive: boolean } {
    const isPositive = value >= 0;
    return {
        text: `${isPositive ? '+' : ''}${value.toFixed(1)}%`,
        isPositive,
    };
}

// Format number with Nigerian thousand separator
export function formatNumber(num: number): string {
    return new Intl.NumberFormat('en-NG').format(num);
}

// Relative time (e.g., "2 days ago")
export function formatRelativeTime(date: Date | string): string {
    const d = typeof date === 'string' ? new Date(date) : date;
    const now = new Date();
    const diffMs = now.getTime() - d.getTime();
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return 'Today';
    if (diffDays === 1) return 'Yesterday';
    if (diffDays < 7) return `${diffDays} days ago`;
    if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
    if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`;
    return `${Math.floor(diffDays / 365)} years ago`;
}
