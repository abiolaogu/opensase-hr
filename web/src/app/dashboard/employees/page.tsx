'use client';

import { Header } from '@/components/layout/Header';
import { EmployeeTable } from '@/components/employees/EmployeeTable';
import { UserPlus, Download, Search, Filter } from 'lucide-react';

// Mock data
const employees = [
    {
        id: '1',
        name: 'Chukwuemeka Okonkwo',
        email: 'chukwuemeka.o@company.ng',
        phone: '+234 803 123 4567',
        department: 'Engineering',
        position: 'Senior Software Engineer',
        status: 'active' as const,
        salary: 850000,
        joinDate: '2021-03-15',
    },
    {
        id: '2',
        name: 'Adesola Bakare',
        email: 'adesola.b@company.ng',
        phone: '+234 805 987 6543',
        department: 'Marketing',
        position: 'Marketing Manager',
        status: 'active' as const,
        salary: 650000,
        joinDate: '2020-08-22',
    },
    {
        id: '3',
        name: 'Ngozi Eze',
        email: 'ngozi.e@company.ng',
        phone: '+234 816 456 7890',
        department: 'HR',
        position: 'HR Business Partner',
        status: 'on_leave' as const,
        salary: 550000,
        joinDate: '2019-06-10',
    },
    {
        id: '4',
        name: 'Oluwaseun Adeyemi',
        email: 'seun.a@company.ng',
        phone: '+234 802 111 2222',
        department: 'Finance',
        position: 'Financial Analyst',
        status: 'active' as const,
        salary: 480000,
        joinDate: '2022-01-05',
    },
    {
        id: '5',
        name: 'Ibrahim Usman',
        email: 'ibrahim.u@company.ng',
        phone: '+234 809 333 4444',
        department: 'Operations',
        position: 'Operations Lead',
        status: 'active' as const,
        salary: 720000,
        joinDate: '2020-11-18',
    },
];

export default function EmployeesPage() {
    return (
        <div className="min-h-screen">
            <Header
                title="Employees"
                subtitle="Manage your workforce"
            />

            <div className="p-6 space-y-6">
                {/* Actions Bar */}
                <div className="flex flex-col sm:flex-row gap-4 justify-between">
                    <div className="flex gap-3">
                        <div className="relative">
                            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-slate-400" />
                            <input
                                type="text"
                                placeholder="Search employees..."
                                className="pl-10 pr-4 py-2.5 w-64 bg-white border border-slate-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                            />
                        </div>
                        <button className="flex items-center gap-2 px-4 py-2.5 bg-white border border-slate-200 rounded-lg text-slate-600 hover:bg-slate-50 transition-colors">
                            <Filter className="h-4 w-4" />
                            Filters
                        </button>
                    </div>
                    <div className="flex gap-3">
                        <button className="flex items-center gap-2 px-4 py-2.5 bg-white border border-slate-200 rounded-lg text-slate-600 hover:bg-slate-50 transition-colors">
                            <Download className="h-4 w-4" />
                            Export
                        </button>
                        <button className="flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-emerald-600 to-teal-600 text-white rounded-lg hover:from-emerald-700 hover:to-teal-700 transition-colors">
                            <UserPlus className="h-4 w-4" />
                            Add Employee
                        </button>
                    </div>
                </div>

                {/* Summary Cards */}
                <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <SummaryCard label="Total Employees" value="156" />
                    <SummaryCard label="Active" value="148" color="emerald" />
                    <SummaryCard label="On Leave" value="6" color="amber" />
                    <SummaryCard label="New This Month" value="4" color="blue" />
                </div>

                {/* Employee Table */}
                <EmployeeTable employees={employees} />
            </div>
        </div>
    );
}

function SummaryCard({
    label,
    value,
    color = 'slate'
}: {
    label: string;
    value: string;
    color?: 'slate' | 'emerald' | 'amber' | 'blue';
}) {
    const colorStyles = {
        slate: 'bg-slate-50 border-slate-200 text-slate-900',
        emerald: 'bg-emerald-50 border-emerald-200 text-emerald-700',
        amber: 'bg-amber-50 border-amber-200 text-amber-700',
        blue: 'bg-blue-50 border-blue-200 text-blue-700',
    };

    return (
        <div className={`p-4 rounded-xl border ${colorStyles[color]}`}>
            <p className="text-sm text-slate-600">{label}</p>
            <p className="text-2xl font-bold mt-1">{value}</p>
        </div>
    );
}
