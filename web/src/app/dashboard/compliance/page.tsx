'use client';

import { Header } from '@/components/layout/Header';
import { formatDate, formatRelativeTime } from '@/lib/nigerian-locale';
import { cn } from '@/lib/utils';
import {
    Shield, FileText, Clock, CheckCircle, AlertTriangle,
    Download, Eye, Bell, Lock, ChevronRight
} from 'lucide-react';

// Compliance items
const complianceChecklist = [
    { id: 'ndpr', category: 'NDPR', item: 'Data Protection Policy', status: 'compliant', lastReview: '2024-01-10' },
    { id: 'paye', category: 'PAYE', item: 'Tax Remittance (January)', status: 'compliant', lastReview: '2024-01-25' },
    { id: 'pencom', category: 'PenCom', item: 'Pension Contribution (January)', status: 'compliant', lastReview: '2024-01-28' },
    { id: 'nhf', category: 'NHF', item: 'Housing Fund (January)', status: 'compliant', lastReview: '2024-01-28' },
    { id: 'nsitf', category: 'NSITF', item: 'Social Insurance (Q4 2023)', status: 'pending', lastReview: '2023-12-15' },
    { id: 'itf', category: 'ITF', item: 'Training Fund (2023)', status: 'compliant', lastReview: '2024-01-05' },
];

const auditLogs = [
    { id: '1', action: 'Employee Created', entity: 'Oluwaseun Adeyemi', user: 'Abiola Ogunsakin', timestamp: '2024-01-30T14:30:00', ip: '102.89.45.123' },
    { id: '2', action: 'Payroll Approved', entity: 'January 2024 Payroll', user: 'Ngozi Eze', timestamp: '2024-01-28T16:45:00', ip: '102.89.45.124' },
    { id: '3', action: 'Leave Approved', entity: 'Chukwuemeka Okonkwo', user: 'Ibrahim Usman', timestamp: '2024-01-28T10:15:00', ip: '102.89.45.125' },
    { id: '4', action: 'Salary Updated', entity: 'Adesola Bakare', user: 'Abiola Ogunsakin', timestamp: '2024-01-27T11:00:00', ip: '102.89.45.123' },
];

const dsrRequests = [
    { id: '1', type: 'Access', subject: 'john.doe@example.com', status: 'pending', dueDate: '2024-02-15', createdAt: '2024-01-16' },
];

export default function CompliancePage() {
    const compliantCount = complianceChecklist.filter(c => c.status === 'compliant').length;
    const pendingCount = complianceChecklist.filter(c => c.status === 'pending').length;

    return (
        <div className="min-h-screen">
            <Header
                title="Compliance & Audit"
                subtitle="NDPR, PAYE, PenCom Compliance"
            />

            <div className="p-6 space-y-6">
                {/* Summary */}
                <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <ComplianceCard
                        icon={CheckCircle}
                        label="Compliant"
                        value={compliantCount.toString()}
                        total={complianceChecklist.length}
                        color="emerald"
                    />
                    <ComplianceCard
                        icon={Clock}
                        label="Pending"
                        value={pendingCount.toString()}
                        color="amber"
                    />
                    <ComplianceCard
                        icon={FileText}
                        label="DSR Requests"
                        value={dsrRequests.length.toString()}
                        color="blue"
                    />
                    <ComplianceCard
                        icon={Shield}
                        label="Audit Logs (30d)"
                        value="1,247"
                        color="purple"
                    />
                </div>

                {/* Compliance Checklist */}
                <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                    <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                        <div className="flex items-center gap-2">
                            <Shield className="h-5 w-5 text-emerald-600" />
                            <h3 className="text-lg font-semibold text-slate-900">Nigerian Regulatory Compliance</h3>
                        </div>
                        <button className="flex items-center gap-1 text-sm text-emerald-600 hover:text-emerald-700 font-medium">
                            <Download className="h-4 w-4" />
                            Export Report
                        </button>
                    </div>
                    <div className="divide-y divide-slate-100">
                        {complianceChecklist.map((item) => (
                            <div key={item.id} className="px-6 py-4 flex items-center justify-between hover:bg-slate-50">
                                <div className="flex items-center gap-4">
                                    <CategoryBadge category={item.category} />
                                    <div>
                                        <p className="font-medium text-slate-900">{item.item}</p>
                                        <p className="text-sm text-slate-500">Last reviewed: {formatDate(item.lastReview)}</p>
                                    </div>
                                </div>
                                <div className="flex items-center gap-4">
                                    {item.status === 'compliant' ? (
                                        <span className="flex items-center gap-1 text-emerald-600">
                                            <CheckCircle className="h-5 w-5" />
                                            <span className="text-sm font-medium">Compliant</span>
                                        </span>
                                    ) : (
                                        <span className="flex items-center gap-1 text-amber-600">
                                            <AlertTriangle className="h-5 w-5" />
                                            <span className="text-sm font-medium">Action Required</span>
                                        </span>
                                    )}
                                    <ChevronRight className="h-5 w-5 text-slate-400" />
                                </div>
                            </div>
                        ))}
                    </div>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    {/* NDPR Data Subject Requests */}
                    <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                            <div className="flex items-center gap-2">
                                <Lock className="h-5 w-5 text-blue-600" />
                                <h3 className="text-lg font-semibold text-slate-900">NDPR Data Requests</h3>
                            </div>
                        </div>
                        {dsrRequests.length === 0 ? (
                            <div className="px-6 py-12 text-center text-slate-500">
                                <Lock className="h-10 w-10 mx-auto mb-2 text-slate-300" />
                                <p>No pending data subject requests</p>
                            </div>
                        ) : (
                            <div className="divide-y divide-slate-100">
                                {dsrRequests.map((dsr) => (
                                    <div key={dsr.id} className="px-6 py-4">
                                        <div className="flex items-center justify-between mb-2">
                                            <span className="px-2 py-0.5 bg-blue-100 text-blue-700 text-xs font-medium rounded">
                                                {dsr.type} Request
                                            </span>
                                            <span className="text-sm text-slate-500">Due: {formatDate(dsr.dueDate)}</span>
                                        </div>
                                        <p className="font-medium text-slate-900">{dsr.subject}</p>
                                        <p className="text-sm text-slate-500">Received {formatRelativeTime(dsr.createdAt)}</p>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>

                    {/* Audit Logs */}
                    <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                            <div className="flex items-center gap-2">
                                <Eye className="h-5 w-5 text-purple-600" />
                                <h3 className="text-lg font-semibold text-slate-900">Recent Audit Logs</h3>
                            </div>
                            <button className="text-sm font-medium text-purple-600 hover:text-purple-700">
                                View All
                            </button>
                        </div>
                        <div className="divide-y divide-slate-100">
                            {auditLogs.map((log) => (
                                <div key={log.id} className="px-6 py-3">
                                    <div className="flex items-center justify-between">
                                        <p className="text-sm font-medium text-slate-900">{log.action}</p>
                                        <span className="text-xs text-slate-400">{formatRelativeTime(log.timestamp)}</span>
                                    </div>
                                    <p className="text-sm text-slate-500">{log.entity}</p>
                                    <p className="text-xs text-slate-400">by {log.user} • {log.ip}</p>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                {/* Compliance Calendar */}
                <div className="bg-gradient-to-r from-slate-800 to-slate-900 rounded-xl p-6 text-white">
                    <div className="flex items-center gap-2 mb-4">
                        <Bell className="h-5 w-5 text-amber-400" />
                        <h3 className="text-lg font-semibold">Upcoming Deadlines</h3>
                    </div>
                    <div className="grid grid-cols-3 gap-4">
                        <DeadlineCard title="PAYE Remittance" date="10/02/2024" days={10} />
                        <DeadlineCard title="Pension Contribution" date="28/02/2024" days={28} />
                        <DeadlineCard title="NSITF Payment" date="15/02/2024" days={15} urgent />
                    </div>
                </div>
            </div>
        </div>
    );
}

function ComplianceCard({ icon: Icon, label, value, total, color }: { icon: any; label: string; value: string; total?: number; color: string }) {
    const bgColor = {
        emerald: 'bg-emerald-100',
        amber: 'bg-amber-100',
        blue: 'bg-blue-100',
        purple: 'bg-purple-100',
    }[color] || 'bg-slate-100';

    const textColor = {
        emerald: 'text-emerald-600',
        amber: 'text-amber-600',
        blue: 'text-blue-600',
        purple: 'text-purple-600',
    }[color] || 'text-slate-600';

    return (
        <div className="bg-white rounded-xl border border-slate-200 p-4">
            <div className="flex items-center gap-3">
                <div className={cn('p-2 rounded-lg', bgColor)}>
                    <Icon className={cn('h-5 w-5', textColor)} />
                </div>
                <div>
                    <p className="text-sm text-slate-500">{label}</p>
                    <p className="text-xl font-bold text-slate-900">
                        {value}
                        {total && <span className="text-sm text-slate-400">/{total}</span>}
                    </p>
                </div>
            </div>
        </div>
    );
}

function CategoryBadge({ category }: { category: string }) {
    const colors: Record<string, string> = {
        NDPR: 'bg-blue-100 text-blue-700',
        PAYE: 'bg-emerald-100 text-emerald-700',
        PenCom: 'bg-purple-100 text-purple-700',
        NHF: 'bg-amber-100 text-amber-700',
        NSITF: 'bg-rose-100 text-rose-700',
        ITF: 'bg-indigo-100 text-indigo-700',
    };

    return (
        <span className={cn('px-2 py-1 text-xs font-medium rounded', colors[category] || 'bg-slate-100 text-slate-600')}>
            {category}
        </span>
    );
}

function DeadlineCard({ title, date, days, urgent }: { title: string; date: string; days: number; urgent?: boolean }) {
    return (
        <div className={cn(
            'p-4 rounded-lg',
            urgent ? 'bg-amber-500/20 border border-amber-500/30' : 'bg-white/10'
        )}>
            <p className="text-sm text-slate-300">{title}</p>
            <p className="text-lg font-semibold">{date}</p>
            <p className={cn('text-sm', urgent ? 'text-amber-400' : 'text-slate-400')}>
                {days} days remaining
            </p>
        </div>
    );
}
