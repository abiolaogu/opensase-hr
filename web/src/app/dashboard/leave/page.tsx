'use client';

import { Header } from '@/components/layout/Header';
import { LeaveRequests } from '@/components/dashboard/LeaveRequests';
import { formatDate } from '@/lib/nigerian-locale';
import { Calendar, Plus, Clock, CheckCircle, XCircle } from 'lucide-react';

// Mock data
const leaveRequests = [
    { id: '1', employeeName: 'Chukwuemeka Okonkwo', leaveType: 'Annual Leave', startDate: '2024-02-05', endDate: '2024-02-09', days: 5, status: 'pending' as const, submittedAt: '2024-01-28T09:30:00' },
    { id: '2', employeeName: 'Adesola Bakare', leaveType: 'Sick Leave', startDate: '2024-02-01', endDate: '2024-02-02', days: 2, status: 'pending' as const, submittedAt: '2024-01-30T14:15:00' },
    { id: '3', employeeName: 'Ngozi Eze', leaveType: 'Maternity Leave', startDate: '2024-02-15', endDate: '2024-05-10', days: 84, status: 'pending' as const, submittedAt: '2024-01-25T11:00:00' },
];

const leaveBalances = [
    { type: 'Annual Leave', entitled: 21, used: 8, pending: 5, color: 'emerald' },
    { type: 'Sick Leave', entitled: 12, used: 2, pending: 0, color: 'blue' },
    { type: 'Compassionate', entitled: 5, used: 0, pending: 0, color: 'purple' },
];

export default function LeavePage() {
    return (
        <div className="min-h-screen">
            <Header
                title="Leave Management"
                subtitle="Nigerian Labour Act Compliant"
            />

            <div className="p-6 space-y-6">
                {/* My Leave Balance */}
                <div className="bg-white rounded-xl border border-slate-200 p-6">
                    <div className="flex items-center justify-between mb-4">
                        <h3 className="text-lg font-semibold text-slate-900">My Leave Balance (2024)</h3>
                        <button className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-emerald-600 to-teal-600 text-white rounded-lg hover:from-emerald-700 hover:to-teal-700 transition-colors text-sm font-medium">
                            <Plus className="h-4 w-4" />
                            Request Leave
                        </button>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        {leaveBalances.map((balance) => (
                            <LeaveBalanceCard key={balance.type} {...balance} />
                        ))}
                    </div>
                </div>

                {/* Leave Stats */}
                <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <StatCard icon={Calendar} label="Total Days" value="38" subtitle="Annual entitlement" />
                    <StatCard icon={CheckCircle} label="Days Used" value="10" subtitle="This year" color="emerald" />
                    <StatCard icon={Clock} label="Pending" value="5" subtitle="Awaiting approval" color="amber" />
                    <StatCard icon={XCircle} label="Available" value="23" subtitle="Days remaining" color="blue" />
                </div>

                {/* Leave Requests */}
                <LeaveRequests requests={leaveRequests} />

                {/* Public Holidays */}
                <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                    <div className="px-6 py-4 border-b border-slate-100">
                        <h3 className="text-lg font-semibold text-slate-900">Nigerian Public Holidays 2024</h3>
                    </div>
                    <div className="p-6 grid grid-cols-2 md:grid-cols-4 gap-4">
                        {[
                            { name: "New Year's Day", date: '01/01/2024' },
                            { name: "Workers Day", date: '01/05/2024' },
                            { name: 'Democracy Day', date: '12/06/2024' },
                            { name: 'Independence Day', date: '01/10/2024' },
                            { name: 'Eid al-Fitr', date: '10/04/2024' },
                            { name: 'Eid al-Adha', date: '17/06/2024' },
                            { name: 'Christmas Day', date: '25/12/2024' },
                            { name: 'Boxing Day', date: '26/12/2024' },
                        ].map((holiday) => (
                            <div key={holiday.name} className="p-3 bg-slate-50 rounded-lg">
                                <p className="font-medium text-slate-900 text-sm">{holiday.name}</p>
                                <p className="text-xs text-slate-500">{holiday.date}</p>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

function LeaveBalanceCard({ type, entitled, used, pending, color }: { type: string; entitled: number; used: number; pending: number; color: string }) {
    const available = entitled - used - pending;
    const usedPercent = (used / entitled) * 100;

    return (
        <div className="p-4 bg-slate-50 rounded-lg">
            <div className="flex items-center justify-between mb-3">
                <p className="font-medium text-slate-900">{type}</p>
                <span className="text-2xl font-bold text-slate-900">{available}</span>
            </div>
            <div className="h-2 bg-slate-200 rounded-full overflow-hidden">
                <div
                    className="h-full bg-emerald-500 rounded-full"
                    style={{ width: `${usedPercent}%` }}
                />
            </div>
            <div className="mt-2 flex justify-between text-xs text-slate-500">
                <span>{used} used</span>
                <span>{entitled} total</span>
            </div>
        </div>
    );
}

function StatCard({ icon: Icon, label, value, subtitle, color = 'slate' }: { icon: any; label: string; value: string; subtitle: string; color?: string }) {
    const bg = {
        slate: 'bg-slate-100',
        emerald: 'bg-emerald-100',
        amber: 'bg-amber-100',
        blue: 'bg-blue-100',
    }[color] || 'bg-slate-100';

    const text = {
        slate: 'text-slate-600',
        emerald: 'text-emerald-600',
        amber: 'text-amber-600',
        blue: 'text-blue-600',
    }[color] || 'text-slate-600';

    return (
        <div className="bg-white rounded-xl border border-slate-200 p-4">
            <div className="flex items-center gap-3">
                <div className={`p-2 rounded-lg ${bg}`}>
                    <Icon className={`h-5 w-5 ${text}`} />
                </div>
                <div>
                    <p className="text-sm text-slate-500">{label}</p>
                    <p className="text-xl font-bold text-slate-900">{value}</p>
                    <p className="text-xs text-slate-400">{subtitle}</p>
                </div>
            </div>
        </div>
    );
}
