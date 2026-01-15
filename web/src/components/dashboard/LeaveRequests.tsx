'use client';

import { formatDate, formatRelativeTime } from '@/lib/nigerian-locale';
import { cn } from '@/lib/utils';
import { Calendar, Clock, User } from 'lucide-react';

interface LeaveRequest {
    id: string;
    employeeName: string;
    leaveType: string;
    startDate: string;
    endDate: string;
    days: number;
    status: 'pending' | 'approved' | 'rejected';
    submittedAt: string;
}

interface LeaveRequestsProps {
    requests: LeaveRequest[];
}

const statusStyles = {
    pending: 'bg-amber-100 text-amber-700 border-amber-200',
    approved: 'bg-emerald-100 text-emerald-700 border-emerald-200',
    rejected: 'bg-red-100 text-red-700 border-red-200',
};

export function LeaveRequests({ requests }: LeaveRequestsProps) {
    return (
        <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
            <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                <div>
                    <h3 className="text-lg font-semibold text-slate-900">Pending Leave Requests</h3>
                    <p className="text-sm text-slate-500">{requests.length} requests awaiting approval</p>
                </div>
                <button className="text-sm font-medium text-emerald-600 hover:text-emerald-700">
                    View All
                </button>
            </div>

            <div className="divide-y divide-slate-100">
                {requests.length === 0 ? (
                    <div className="px-6 py-12 text-center">
                        <Calendar className="h-12 w-12 text-slate-300 mx-auto mb-3" />
                        <p className="text-slate-500">No pending leave requests</p>
                    </div>
                ) : (
                    requests.map((request) => (
                        <div key={request.id} className="px-6 py-4 hover:bg-slate-50 transition-colors">
                            <div className="flex items-start justify-between">
                                <div className="flex items-start gap-3">
                                    <div className="w-10 h-10 rounded-full bg-gradient-to-br from-slate-200 to-slate-300 flex items-center justify-center">
                                        <User className="h-5 w-5 text-slate-600" />
                                    </div>
                                    <div>
                                        <p className="font-medium text-slate-900">{request.employeeName}</p>
                                        <p className="text-sm text-slate-500">
                                            {request.leaveType} • {request.days} day{request.days > 1 ? 's' : ''}
                                        </p>
                                        <div className="flex items-center gap-4 mt-1 text-xs text-slate-400">
                                            <span className="flex items-center gap-1">
                                                <Calendar className="h-3 w-3" />
                                                {formatDate(request.startDate)} - {formatDate(request.endDate)}
                                            </span>
                                            <span className="flex items-center gap-1">
                                                <Clock className="h-3 w-3" />
                                                {formatRelativeTime(request.submittedAt)}
                                            </span>
                                        </div>
                                    </div>
                                </div>

                                <div className="flex items-center gap-2">
                                    {request.status === 'pending' && (
                                        <>
                                            <button className="px-3 py-1.5 text-xs font-medium text-red-600 hover:bg-red-50 rounded-lg transition-colors">
                                                Reject
                                            </button>
                                            <button className="px-3 py-1.5 text-xs font-medium text-white bg-emerald-600 hover:bg-emerald-700 rounded-lg transition-colors">
                                                Approve
                                            </button>
                                        </>
                                    )}
                                    {request.status !== 'pending' && (
                                        <span className={cn(
                                            'px-2 py-1 text-xs font-medium rounded-full border capitalize',
                                            statusStyles[request.status]
                                        )}>
                                            {request.status}
                                        </span>
                                    )}
                                </div>
                            </div>
                        </div>
                    ))
                )}
            </div>
        </div>
    );
}
