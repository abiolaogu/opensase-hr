'use client';

import { formatNaira, formatDate } from '@/lib/nigerian-locale';
import { cn } from '@/lib/utils';
import { User, MoreVertical, Mail, Phone } from 'lucide-react';

interface Employee {
    id: string;
    name: string;
    email: string;
    phone: string;
    department: string;
    position: string;
    status: 'active' | 'on_leave' | 'terminated';
    salary: number;
    joinDate: string;
}

interface EmployeeTableProps {
    employees: Employee[];
}

const statusStyles = {
    active: 'bg-emerald-100 text-emerald-700',
    on_leave: 'bg-amber-100 text-amber-700',
    terminated: 'bg-red-100 text-red-700',
};

const statusLabels = {
    active: 'Active',
    on_leave: 'On Leave',
    terminated: 'Terminated',
};

export function EmployeeTable({ employees }: EmployeeTableProps) {
    return (
        <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
            <div className="overflow-x-auto">
                <table className="w-full">
                    <thead>
                        <tr className="bg-slate-50 border-b border-slate-200">
                            <th className="px-6 py-4 text-left text-xs font-semibold text-slate-600 uppercase tracking-wider">
                                Employee
                            </th>
                            <th className="px-6 py-4 text-left text-xs font-semibold text-slate-600 uppercase tracking-wider">
                                Department
                            </th>
                            <th className="px-6 py-4 text-left text-xs font-semibold text-slate-600 uppercase tracking-wider">
                                Position
                            </th>
                            <th className="px-6 py-4 text-left text-xs font-semibold text-slate-600 uppercase tracking-wider">
                                Salary
                            </th>
                            <th className="px-6 py-4 text-left text-xs font-semibold text-slate-600 uppercase tracking-wider">
                                Status
                            </th>
                            <th className="px-6 py-4 text-left text-xs font-semibold text-slate-600 uppercase tracking-wider">
                                Join Date
                            </th>
                            <th className="px-6 py-4"></th>
                        </tr>
                    </thead>
                    <tbody className="divide-y divide-slate-100">
                        {employees.map((employee) => (
                            <tr key={employee.id} className="hover:bg-slate-50 transition-colors">
                                <td className="px-6 py-4">
                                    <div className="flex items-center gap-3">
                                        <div className="w-10 h-10 rounded-full bg-gradient-to-br from-emerald-400 to-teal-500 flex items-center justify-center text-white font-medium">
                                            {employee.name.split(' ').map(n => n[0]).join('')}
                                        </div>
                                        <div>
                                            <p className="font-medium text-slate-900">{employee.name}</p>
                                            <p className="text-sm text-slate-500">{employee.email}</p>
                                        </div>
                                    </div>
                                </td>
                                <td className="px-6 py-4 text-sm text-slate-600">
                                    {employee.department}
                                </td>
                                <td className="px-6 py-4 text-sm text-slate-600">
                                    {employee.position}
                                </td>
                                <td className="px-6 py-4 text-sm font-medium text-slate-900">
                                    {formatNaira(employee.salary)}/mo
                                </td>
                                <td className="px-6 py-4">
                                    <span className={cn(
                                        'px-2 py-1 text-xs font-medium rounded-full',
                                        statusStyles[employee.status]
                                    )}>
                                        {statusLabels[employee.status]}
                                    </span>
                                </td>
                                <td className="px-6 py-4 text-sm text-slate-600">
                                    {formatDate(employee.joinDate)}
                                </td>
                                <td className="px-6 py-4">
                                    <button className="p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-100 rounded-lg transition-colors">
                                        <MoreVertical className="h-4 w-4" />
                                    </button>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}
