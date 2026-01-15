'use client';

import { formatNaira, formatDate } from '@/lib/nigerian-locale';
import { cn } from '@/lib/utils';

interface PayrollOverviewProps {
    month: string;
    totalGross: number;
    totalNet: number;
    totalPaye: number;
    totalPension: number;
    totalNhf: number;
    employeeCount: number;
    status: 'draft' | 'processing' | 'approved' | 'paid';
}

const statusStyles = {
    draft: 'bg-slate-100 text-slate-600',
    processing: 'bg-amber-100 text-amber-700',
    approved: 'bg-emerald-100 text-emerald-700',
    paid: 'bg-blue-100 text-blue-700',
};

export function PayrollOverview({
    month,
    totalGross,
    totalNet,
    totalPaye,
    totalPension,
    totalNhf,
    employeeCount,
    status,
}: PayrollOverviewProps) {
    return (
        <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
            {/* Header */}
            <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                <div>
                    <h3 className="text-lg font-semibold text-slate-900">Payroll Overview</h3>
                    <p className="text-sm text-slate-500">{month}</p>
                </div>
                <span className={cn(
                    'px-3 py-1 rounded-full text-xs font-medium capitalize',
                    statusStyles[status]
                )}>
                    {status}
                </span>
            </div>

            {/* Summary Grid */}
            <div className="p-6 grid grid-cols-2 gap-6">
                <div>
                    <p className="text-sm text-slate-500">Gross Payroll</p>
                    <p className="text-2xl font-bold text-slate-900">{formatNaira(totalGross)}</p>
                </div>
                <div>
                    <p className="text-sm text-slate-500">Net Payroll</p>
                    <p className="text-2xl font-bold text-emerald-600">{formatNaira(totalNet)}</p>
                </div>
            </div>

            {/* Deductions Breakdown */}
            <div className="px-6 py-4 bg-slate-50 border-t border-slate-100">
                <h4 className="text-sm font-medium text-slate-700 mb-3">Statutory Deductions</h4>
                <div className="space-y-2">
                    <div className="flex justify-between text-sm">
                        <span className="text-slate-500">PAYE Tax</span>
                        <span className="font-medium text-slate-900">{formatNaira(totalPaye)}</span>
                    </div>
                    <div className="flex justify-between text-sm">
                        <span className="text-slate-500">Pension (Employee)</span>
                        <span className="font-medium text-slate-900">{formatNaira(totalPension)}</span>
                    </div>
                    <div className="flex justify-between text-sm">
                        <span className="text-slate-500">NHF</span>
                        <span className="font-medium text-slate-900">{formatNaira(totalNhf)}</span>
                    </div>
                </div>
            </div>

            {/* Footer */}
            <div className="px-6 py-4 border-t border-slate-100 flex items-center justify-between">
                <span className="text-sm text-slate-500">
                    {employeeCount} employees
                </span>
                <button className="text-sm font-medium text-emerald-600 hover:text-emerald-700">
                    View Details →
                </button>
            </div>
        </div>
    );
}
