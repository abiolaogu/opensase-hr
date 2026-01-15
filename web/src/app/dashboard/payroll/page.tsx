'use client';

import { Header } from '@/components/layout/Header';
import { TaxCalculator } from '@/components/payroll/TaxCalculator';
import { PayrollOverview } from '@/components/dashboard/PayrollOverview';
import { formatNaira, formatDate } from '@/lib/nigerian-locale';
import { Play, Download, History, ChevronRight } from 'lucide-react';

// Mock payroll history
const payrollHistory = [
    { id: '1', month: 'December 2023', status: 'paid' as const, totalNet: 31850000, employeeCount: 154, paidAt: '2023-12-28' },
    { id: '2', month: 'November 2023', status: 'paid' as const, totalNet: 31650000, employeeCount: 152, paidAt: '2023-11-29' },
    { id: '3', month: 'October 2023', status: 'paid' as const, totalNet: 30980000, employeeCount: 150, paidAt: '2023-10-30' },
];

export default function PayrollPage() {
    return (
        <div className="min-h-screen">
            <Header
                title="Payroll"
                subtitle="Nigerian PAYE Tax Compliant"
            />

            <div className="p-6 space-y-6">
                {/* Actions */}
                <div className="flex gap-4">
                    <button className="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-emerald-600 to-teal-600 text-white rounded-lg hover:from-emerald-700 hover:to-teal-700 transition-colors font-medium">
                        <Play className="h-5 w-5" />
                        Run January 2024 Payroll
                    </button>
                    <button className="flex items-center gap-2 px-6 py-3 bg-white border border-slate-200 text-slate-700 rounded-lg hover:bg-slate-50 transition-colors">
                        <Download className="h-5 w-5" />
                        Export Reports
                    </button>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    {/* Current Payroll */}
                    <div className="lg:col-span-2">
                        <PayrollOverview
                            month="January 2024"
                            totalGross={45600000}
                            totalNet={32480000}
                            totalPaye={8540000}
                            totalPension={3648000}
                            totalNhf={684000}
                            employeeCount={156}
                            status="approved"
                        />
                    </div>

                    {/* Tax Calculator */}
                    <div>
                        <TaxCalculator />
                    </div>
                </div>

                {/* Payroll History */}
                <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                    <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                        <div className="flex items-center gap-2">
                            <History className="h-5 w-5 text-slate-400" />
                            <h3 className="text-lg font-semibold text-slate-900">Payroll History</h3>
                        </div>
                        <button className="text-sm font-medium text-emerald-600 hover:text-emerald-700">
                            View All
                        </button>
                    </div>

                    <div className="divide-y divide-slate-100">
                        {payrollHistory.map((payroll) => (
                            <div key={payroll.id} className="px-6 py-4 flex items-center justify-between hover:bg-slate-50 transition-colors cursor-pointer">
                                <div>
                                    <p className="font-medium text-slate-900">{payroll.month}</p>
                                    <p className="text-sm text-slate-500">{payroll.employeeCount} employees</p>
                                </div>
                                <div className="text-right flex items-center gap-4">
                                    <div>
                                        <p className="font-semibold text-slate-900">{formatNaira(payroll.totalNet)}</p>
                                        <p className="text-xs text-slate-500">Paid {formatDate(payroll.paidAt)}</p>
                                    </div>
                                    <ChevronRight className="h-5 w-5 text-slate-400" />
                                </div>
                            </div>
                        ))}
                    </div>
                </div>

                {/* Statutory Compliance */}
                <div className="bg-white rounded-xl border border-slate-200 p-6">
                    <h3 className="text-lg font-semibold text-slate-900 mb-4">Statutory Compliance</h3>
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                        <ComplianceCard title="PAYE Tax" description="FIRS remittance" status="up-to-date" />
                        <ComplianceCard title="PenCom" description="Pension contributions" status="up-to-date" />
                        <ComplianceCard title="NHF" description="Housing fund" status="up-to-date" />
                        <ComplianceCard title="NSITF" description="Social insurance" status="pending" />
                    </div>
                </div>
            </div>
        </div>
    );
}

function ComplianceCard({
    title,
    description,
    status,
}: {
    title: string;
    description: string;
    status: 'up-to-date' | 'pending' | 'overdue';
}) {
    const statusStyles = {
        'up-to-date': 'bg-emerald-100 text-emerald-700',
        'pending': 'bg-amber-100 text-amber-700',
        'overdue': 'bg-red-100 text-red-700',
    };

    return (
        <div className="p-4 bg-slate-50 rounded-lg">
            <div className="flex items-center justify-between mb-2">
                <p className="font-medium text-slate-900">{title}</p>
                <span className={`px-2 py-0.5 text-xs font-medium rounded-full capitalize ${statusStyles[status]}`}>
                    {status.replace('-', ' ')}
                </span>
            </div>
            <p className="text-sm text-slate-500">{description}</p>
        </div>
    );
}
