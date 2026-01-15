'use client';

import { Header } from '@/components/layout/Header';
import { StatsCard } from '@/components/dashboard/StatsCard';
import { PayrollOverview } from '@/components/dashboard/PayrollOverview';
import { LeaveRequests } from '@/components/dashboard/LeaveRequests';
import { TaxCalculator } from '@/components/payroll/TaxCalculator';
import { Users, Banknote, Calendar, Target, Clock, Briefcase } from 'lucide-react';

// Mock data - in production, fetch from API
const stats = [
    { title: 'Total Employees', value: 156, change: 4.2, changeLabel: 'vs last month', icon: Users, iconColor: 'text-emerald-500' as const },
    { title: 'Monthly Payroll', value: 45600000, change: 2.5, changeLabel: 'vs last month', icon: Banknote, iconColor: 'text-blue-500' as const, isCurrency: true },
    { title: 'On Leave Today', value: 8, icon: Calendar, iconColor: 'text-purple-500' as const },
    { title: 'Open Positions', value: 12, change: -15, changeLabel: 'vs last month', icon: Briefcase, iconColor: 'text-orange-500' as const },
];

const pendingLeaveRequests = [
    {
        id: '1',
        employeeName: 'Chukwuemeka Okonkwo',
        leaveType: 'Annual Leave',
        startDate: '2024-02-05',
        endDate: '2024-02-09',
        days: 5,
        status: 'pending' as const,
        submittedAt: '2024-01-28T09:30:00',
    },
    {
        id: '2',
        employeeName: 'Adesola Bakare',
        leaveType: 'Sick Leave',
        startDate: '2024-02-01',
        endDate: '2024-02-02',
        days: 2,
        status: 'pending' as const,
        submittedAt: '2024-01-30T14:15:00',
    },
    {
        id: '3',
        employeeName: 'Ngozi Eze',
        leaveType: 'Maternity Leave',
        startDate: '2024-02-15',
        endDate: '2024-05-10',
        days: 84,
        status: 'pending' as const,
        submittedAt: '2024-01-25T11:00:00',
    },
];

export default function DashboardPage() {
    return (
        <div className="min-h-screen">
            <Header
                title="Dashboard"
                subtitle="Welcome back, Abiola"
            />

            <div className="p-6 space-y-6">
                {/* Stats Grid */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    {stats.map((stat) => (
                        <StatsCard
                            key={stat.title}
                            title={stat.title}
                            value={stat.value}
                            change={stat.change}
                            changeLabel={stat.changeLabel}
                            icon={stat.icon}
                            iconColor={stat.iconColor}
                            isCurrency={stat.isCurrency}
                        />
                    ))}
                </div>

                {/* Main Content Grid */}
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    {/* Left Column - Payroll & Tax Calculator */}
                    <div className="lg:col-span-2 space-y-6">
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
                        <TaxCalculator />
                    </div>

                    {/* Right Column - Leave Requests */}
                    <div>
                        <LeaveRequests requests={pendingLeaveRequests} />
                    </div>
                </div>

                {/* Quick Actions */}
                <div className="bg-white rounded-xl border border-slate-200 p-6">
                    <h3 className="text-lg font-semibold text-slate-900 mb-4">Quick Actions</h3>
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                        <QuickAction icon={Users} label="Add Employee" href="/dashboard/employees/new" color="emerald" />
                        <QuickAction icon={Banknote} label="Run Payroll" href="/dashboard/payroll/run" color="blue" />
                        <QuickAction icon={Calendar} label="Request Leave" href="/dashboard/leave/request" color="purple" />
                        <QuickAction icon={Target} label="Start Review" href="/dashboard/performance/new" color="orange" />
                    </div>
                </div>
            </div>
        </div>
    );
}

function QuickAction({
    icon: Icon,
    label,
    href,
    color
}: {
    icon: any;
    label: string;
    href: string;
    color: 'emerald' | 'blue' | 'purple' | 'orange';
}) {
    const colorStyles = {
        emerald: 'from-emerald-50 to-teal-50 hover:from-emerald-100 hover:to-teal-100 text-emerald-700',
        blue: 'from-blue-50 to-indigo-50 hover:from-blue-100 hover:to-indigo-100 text-blue-700',
        purple: 'from-purple-50 to-pink-50 hover:from-purple-100 hover:to-pink-100 text-purple-700',
        orange: 'from-orange-50 to-amber-50 hover:from-orange-100 hover:to-amber-100 text-orange-700',
    };

    return (
        <a
            href={href}
            className={`flex flex-col items-center gap-2 p-4 rounded-xl bg-gradient-to-br transition-colors ${colorStyles[color]}`}
        >
            <Icon className="h-6 w-6" />
            <span className="text-sm font-medium">{label}</span>
        </a>
    );
}
