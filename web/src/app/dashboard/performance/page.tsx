'use client';

import { Header } from '@/components/layout/Header';
import { formatDate } from '@/lib/nigerian-locale';
import { cn } from '@/lib/utils';
import {
    Target, Plus, Star, TrendingUp, Clock, CheckCircle,
    User, ChevronRight, BarChart3
} from 'lucide-react';

// Mock data
const activeCycle = {
    name: 'Q1 2024 Performance Review',
    type: 'Quarterly',
    startDate: '2024-01-01',
    endDate: '2024-03-31',
    status: 'active',
    selfReviewDeadline: '2024-03-15',
    managerReviewDeadline: '2024-03-25',
    progress: {
        total: 156,
        selfSubmitted: 89,
        managerReviewed: 45,
        completed: 32,
    }
};

const myGoals = [
    { id: '1', title: 'Increase customer retention by 15%', weight: 30, progress: 65, status: 'on_track' },
    { id: '2', title: 'Complete 3 professional certifications', weight: 20, progress: 33, status: 'on_track' },
    { id: '3', title: 'Mentor 2 junior team members', weight: 25, progress: 100, status: 'completed' },
    { id: '4', title: 'Reduce operational costs by 10%', weight: 25, progress: 40, status: 'at_risk' },
];

const teamReviews = [
    { id: '1', name: 'Chukwuemeka Okonkwo', position: 'Senior Engineer', selfRating: 4.2, status: 'pending_manager' },
    { id: '2', name: 'Adesola Bakare', position: 'Marketing Lead', selfRating: 3.8, status: 'pending_manager' },
    { id: '3', name: 'Ibrahim Usman', position: 'Operations Lead', selfRating: 4.5, status: 'completed' },
];

const ratingCategories = [
    { min: 1, max: 2, label: 'Needs Improvement', color: 'red' },
    { min: 2, max: 3, label: 'Meets Some', color: 'orange' },
    { min: 3, max: 4, label: 'Meets Expectations', color: 'emerald' },
    { min: 4, max: 4.5, label: 'Exceeds', color: 'blue' },
    { min: 4.5, max: 5, label: 'Outstanding', color: 'purple' },
];

export default function PerformancePage() {
    return (
        <div className="min-h-screen">
            <Header
                title="Performance Reviews"
                subtitle="Goals, Competencies & 360° Feedback"
            />

            <div className="p-6 space-y-6">
                {/* Active Cycle */}
                <div className="bg-gradient-to-r from-purple-600 to-indigo-600 rounded-xl p-6 text-white">
                    <div className="flex items-center justify-between mb-4">
                        <div>
                            <p className="text-purple-200 text-sm">Active Review Cycle</p>
                            <h2 className="text-2xl font-bold">{activeCycle.name}</h2>
                        </div>
                        <span className="px-3 py-1 bg-white/20 rounded-full text-sm">
                            {activeCycle.type}
                        </span>
                    </div>

                    <div className="grid grid-cols-4 gap-4 mt-6">
                        <CycleProgressCard
                            label="Total Employees"
                            value={activeCycle.progress.total}
                            icon={User}
                        />
                        <CycleProgressCard
                            label="Self Reviews"
                            value={activeCycle.progress.selfSubmitted}
                            total={activeCycle.progress.total}
                            icon={CheckCircle}
                        />
                        <CycleProgressCard
                            label="Manager Reviews"
                            value={activeCycle.progress.managerReviewed}
                            total={activeCycle.progress.total}
                            icon={Star}
                        />
                        <CycleProgressCard
                            label="Completed"
                            value={activeCycle.progress.completed}
                            total={activeCycle.progress.total}
                            icon={Target}
                        />
                    </div>

                    <div className="mt-4 flex gap-4 text-sm text-purple-200">
                        <span>Self Review Deadline: {formatDate(activeCycle.selfReviewDeadline)}</span>
                        <span>•</span>
                        <span>Manager Review Deadline: {formatDate(activeCycle.managerReviewDeadline)}</span>
                    </div>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                    {/* My Goals */}
                    <div className="lg:col-span-2 bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                            <div className="flex items-center gap-2">
                                <Target className="h-5 w-5 text-purple-600" />
                                <h3 className="text-lg font-semibold text-slate-900">My Goals</h3>
                            </div>
                            <button className="flex items-center gap-1 text-sm text-purple-600 hover:text-purple-700 font-medium">
                                <Plus className="h-4 w-4" />
                                Add Goal
                            </button>
                        </div>
                        <div className="divide-y divide-slate-100">
                            {myGoals.map((goal) => (
                                <div key={goal.id} className="px-6 py-4 hover:bg-slate-50 transition-colors">
                                    <div className="flex items-start justify-between mb-2">
                                        <div className="flex-1">
                                            <p className="font-medium text-slate-900">{goal.title}</p>
                                            <p className="text-sm text-slate-500">Weight: {goal.weight}%</p>
                                        </div>
                                        <GoalStatusBadge status={goal.status} />
                                    </div>
                                    <div className="mt-2">
                                        <div className="flex items-center justify-between text-sm mb-1">
                                            <span className="text-slate-500">Progress</span>
                                            <span className="font-medium text-slate-900">{goal.progress}%</span>
                                        </div>
                                        <div className="h-2 bg-slate-200 rounded-full overflow-hidden">
                                            <div
                                                className={cn(
                                                    "h-full rounded-full transition-all",
                                                    goal.status === 'completed' && 'bg-emerald-500',
                                                    goal.status === 'on_track' && 'bg-blue-500',
                                                    goal.status === 'at_risk' && 'bg-amber-500'
                                                )}
                                                style={{ width: `${goal.progress}%` }}
                                            />
                                        </div>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>

                    {/* Rating Scale */}
                    <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100">
                            <div className="flex items-center gap-2">
                                <BarChart3 className="h-5 w-5 text-slate-400" />
                                <h3 className="text-lg font-semibold text-slate-900">Rating Scale</h3>
                            </div>
                        </div>
                        <div className="p-6 space-y-3">
                            {ratingCategories.map((cat) => (
                                <div key={cat.label} className="flex items-center gap-3">
                                    <div className={cn(
                                        'w-3 h-3 rounded-full',
                                        cat.color === 'red' && 'bg-red-500',
                                        cat.color === 'orange' && 'bg-orange-500',
                                        cat.color === 'emerald' && 'bg-emerald-500',
                                        cat.color === 'blue' && 'bg-blue-500',
                                        cat.color === 'purple' && 'bg-purple-500',
                                    )} />
                                    <div className="flex-1">
                                        <p className="text-sm font-medium text-slate-900">{cat.label}</p>
                                        <p className="text-xs text-slate-500">{cat.min} - {cat.max}</p>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                {/* Team Reviews (for managers) */}
                <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                    <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                        <div className="flex items-center gap-2">
                            <Star className="h-5 w-5 text-amber-500" />
                            <h3 className="text-lg font-semibold text-slate-900">Team Reviews Pending</h3>
                        </div>
                        <button className="text-sm font-medium text-purple-600 hover:text-purple-700">
                            View All
                        </button>
                    </div>
                    <div className="divide-y divide-slate-100">
                        {teamReviews.map((review) => (
                            <div key={review.id} className="px-6 py-4 flex items-center justify-between hover:bg-slate-50 transition-colors cursor-pointer">
                                <div className="flex items-center gap-3">
                                    <div className="w-10 h-10 rounded-full bg-gradient-to-br from-purple-400 to-indigo-500 flex items-center justify-center text-white font-medium">
                                        {review.name.split(' ').map(n => n[0]).join('')}
                                    </div>
                                    <div>
                                        <p className="font-medium text-slate-900">{review.name}</p>
                                        <p className="text-sm text-slate-500">{review.position}</p>
                                    </div>
                                </div>
                                <div className="flex items-center gap-4">
                                    <div className="text-right">
                                        <p className="text-sm text-slate-500">Self Rating</p>
                                        <p className="font-semibold text-slate-900">{review.selfRating}/5.0</p>
                                    </div>
                                    <ReviewStatusBadge status={review.status} />
                                    <ChevronRight className="h-5 w-5 text-slate-400" />
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

function CycleProgressCard({ label, value, total, icon: Icon }: { label: string; value: number; total?: number; icon: any }) {
    return (
        <div className="bg-white/10 rounded-lg p-3">
            <div className="flex items-center gap-2 mb-1">
                <Icon className="h-4 w-4 text-purple-200" />
                <p className="text-sm text-purple-200">{label}</p>
            </div>
            <p className="text-2xl font-bold">
                {value}
                {total && <span className="text-lg text-purple-200">/{total}</span>}
            </p>
        </div>
    );
}

function GoalStatusBadge({ status }: { status: string }) {
    const styles = {
        completed: 'bg-emerald-100 text-emerald-700',
        on_track: 'bg-blue-100 text-blue-700',
        at_risk: 'bg-amber-100 text-amber-700',
    }[status] || 'bg-slate-100 text-slate-700';

    const labels = {
        completed: 'Completed',
        on_track: 'On Track',
        at_risk: 'At Risk',
    }[status] || status;

    return (
        <span className={cn('px-2 py-1 text-xs font-medium rounded-full', styles)}>
            {labels}
        </span>
    );
}

function ReviewStatusBadge({ status }: { status: string }) {
    const styles = {
        completed: 'bg-emerald-100 text-emerald-700',
        pending_manager: 'bg-amber-100 text-amber-700',
        pending_self: 'bg-blue-100 text-blue-700',
    }[status] || 'bg-slate-100 text-slate-700';

    const labels = {
        completed: 'Completed',
        pending_manager: 'Awaiting Review',
        pending_self: 'Self Review Pending',
    }[status] || status;

    return (
        <span className={cn('px-2 py-1 text-xs font-medium rounded-full', styles)}>
            {labels}
        </span>
    );
}
