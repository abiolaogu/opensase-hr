'use client';

import { Header } from '@/components/layout/Header';
import { formatNaira, formatDate, formatRelativeTime } from '@/lib/nigerian-locale';
import { cn } from '@/lib/utils';
import {
    Briefcase, Plus, Users, Eye, Search, Filter,
    ThumbsUp, ThumbsDown, Clock, CheckCircle, XCircle,
    FileText, Sparkles, ChevronRight, MapPin
} from 'lucide-react';

// Mock data
const jobPostings = [
    {
        id: '1',
        title: 'Senior Software Engineer',
        department: 'Engineering',
        location: 'Lagos, Nigeria',
        type: 'Full-time',
        salaryMin: 800000,
        salaryMax: 1200000,
        status: 'published',
        applications: 45,
        newApplications: 12,
        postedDate: '2024-01-10',
    },
    {
        id: '2',
        title: 'Marketing Manager',
        department: 'Marketing',
        location: 'Lagos, Nigeria',
        type: 'Full-time',
        salaryMin: 600000,
        salaryMax: 900000,
        status: 'published',
        applications: 28,
        newApplications: 5,
        postedDate: '2024-01-15',
    },
    {
        id: '3',
        title: 'Financial Analyst',
        department: 'Finance',
        location: 'Remote (Nigeria)',
        type: 'Full-time',
        salaryMin: 450000,
        salaryMax: 650000,
        status: 'draft',
        applications: 0,
        newApplications: 0,
        postedDate: null,
    },
];

const recentApplications = [
    {
        id: '1',
        name: 'Oluwaseun Adeyemi',
        position: 'Senior Software Engineer',
        aiScore: 92,
        aiRecommendation: 'strong_yes',
        stage: 'interview',
        appliedAt: '2024-01-28T10:30:00',
        skills: ['Rust', 'TypeScript', 'PostgreSQL'],
    },
    {
        id: '2',
        name: 'Chidinma Okafor',
        position: 'Senior Software Engineer',
        aiScore: 78,
        aiRecommendation: 'yes',
        stage: 'screening',
        appliedAt: '2024-01-29T14:15:00',
        skills: ['Python', 'Django', 'AWS'],
    },
    {
        id: '3',
        name: 'Babatunde Afolabi',
        position: 'Marketing Manager',
        aiScore: 65,
        aiRecommendation: 'maybe',
        stage: 'received',
        appliedAt: '2024-01-30T09:00:00',
        skills: ['Digital Marketing', 'SEO', 'Content'],
    },
];

const pipelineStats = [
    { stage: 'Received', count: 45, color: 'slate' },
    { stage: 'Screening', count: 23, color: 'blue' },
    { stage: 'Interview', count: 12, color: 'purple' },
    { stage: 'Offer', count: 3, color: 'amber' },
    { stage: 'Hired', count: 2, color: 'emerald' },
];

export default function RecruitmentPage() {
    return (
        <div className="min-h-screen">
            <Header
                title="Recruitment"
                subtitle="AI-Powered Candidate Screening"
            />

            <div className="p-6 space-y-6">
                {/* Stats */}
                <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <StatCard icon={Briefcase} label="Open Positions" value="12" color="blue" />
                    <StatCard icon={Users} label="Total Applications" value="156" color="purple" />
                    <StatCard icon={Sparkles} label="AI Scored" value="142" color="emerald" />
                    <StatCard icon={CheckCircle} label="Hired This Month" value="4" color="amber" />
                </div>

                {/* Pipeline */}
                <div className="bg-white rounded-xl border border-slate-200 p-6">
                    <h3 className="text-lg font-semibold text-slate-900 mb-4">Hiring Pipeline</h3>
                    <div className="flex items-center gap-2">
                        {pipelineStats.map((stage, i) => (
                            <div key={stage.stage} className="flex-1 relative">
                                <div className={cn(
                                    'text-center p-4 rounded-lg',
                                    stage.color === 'slate' && 'bg-slate-100',
                                    stage.color === 'blue' && 'bg-blue-100',
                                    stage.color === 'purple' && 'bg-purple-100',
                                    stage.color === 'amber' && 'bg-amber-100',
                                    stage.color === 'emerald' && 'bg-emerald-100',
                                )}>
                                    <p className="text-2xl font-bold text-slate-900">{stage.count}</p>
                                    <p className="text-sm text-slate-600">{stage.stage}</p>
                                </div>
                                {i < pipelineStats.length - 1 && (
                                    <ChevronRight className="absolute right-0 top-1/2 -translate-y-1/2 translate-x-1/2 h-5 w-5 text-slate-400 z-10" />
                                )}
                            </div>
                        ))}
                    </div>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    {/* Job Postings */}
                    <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                            <h3 className="text-lg font-semibold text-slate-900">Job Postings</h3>
                            <button className="flex items-center gap-1 px-3 py-1.5 bg-gradient-to-r from-blue-600 to-indigo-600 text-white rounded-lg text-sm font-medium hover:from-blue-700 hover:to-indigo-700">
                                <Plus className="h-4 w-4" />
                                Post Job
                            </button>
                        </div>
                        <div className="divide-y divide-slate-100">
                            {jobPostings.map((job) => (
                                <div key={job.id} className="px-6 py-4 hover:bg-slate-50 transition-colors cursor-pointer">
                                    <div className="flex items-start justify-between mb-2">
                                        <div>
                                            <p className="font-medium text-slate-900">{job.title}</p>
                                            <div className="flex items-center gap-2 text-sm text-slate-500 mt-1">
                                                <span>{job.department}</span>
                                                <span>•</span>
                                                <span className="flex items-center gap-1">
                                                    <MapPin className="h-3 w-3" />
                                                    {job.location}
                                                </span>
                                            </div>
                                        </div>
                                        <JobStatusBadge status={job.status} />
                                    </div>
                                    <div className="flex items-center justify-between mt-3">
                                        <span className="text-sm text-slate-600">
                                            {formatNaira(job.salaryMin, false)} - {formatNaira(job.salaryMax, false)}/mo
                                        </span>
                                        {job.status === 'published' && (
                                            <span className="flex items-center gap-1 text-sm">
                                                <Users className="h-4 w-4 text-slate-400" />
                                                <span className="text-slate-600">{job.applications} applications</span>
                                                {job.newApplications > 0 && (
                                                    <span className="ml-1 px-1.5 py-0.5 bg-blue-100 text-blue-700 text-xs rounded-full">
                                                        +{job.newApplications} new
                                                    </span>
                                                )}
                                            </span>
                                        )}
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>

                    {/* Recent Applications with AI Scoring */}
                    <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                            <div className="flex items-center gap-2">
                                <Sparkles className="h-5 w-5 text-purple-500" />
                                <h3 className="text-lg font-semibold text-slate-900">AI-Scored Applications</h3>
                            </div>
                            <button className="text-sm font-medium text-blue-600 hover:text-blue-700">
                                View All
                            </button>
                        </div>
                        <div className="divide-y divide-slate-100">
                            {recentApplications.map((app) => (
                                <div key={app.id} className="px-6 py-4 hover:bg-slate-50 transition-colors">
                                    <div className="flex items-start justify-between mb-2">
                                        <div className="flex items-center gap-3">
                                            <div className="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-indigo-500 flex items-center justify-center text-white font-medium">
                                                {app.name.split(' ').map(n => n[0]).join('')}
                                            </div>
                                            <div>
                                                <p className="font-medium text-slate-900">{app.name}</p>
                                                <p className="text-sm text-slate-500">{app.position}</p>
                                            </div>
                                        </div>
                                        <AiScoreBadge score={app.aiScore} recommendation={app.aiRecommendation} />
                                    </div>
                                    <div className="flex items-center gap-2 mt-2">
                                        {app.skills.slice(0, 3).map((skill) => (
                                            <span key={skill} className="px-2 py-0.5 bg-slate-100 text-slate-600 text-xs rounded">
                                                {skill}
                                            </span>
                                        ))}
                                    </div>
                                    <div className="flex items-center justify-between mt-3">
                                        <StageBadge stage={app.stage} />
                                        <span className="text-xs text-slate-400">{formatRelativeTime(app.appliedAt)}</span>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

function StatCard({ icon: Icon, label, value, color }: { icon: any; label: string; value: string; color: string }) {
    const bgColor = {
        blue: 'bg-blue-100',
        purple: 'bg-purple-100',
        emerald: 'bg-emerald-100',
        amber: 'bg-amber-100',
    }[color] || 'bg-slate-100';

    const textColor = {
        blue: 'text-blue-600',
        purple: 'text-purple-600',
        emerald: 'text-emerald-600',
        amber: 'text-amber-600',
    }[color] || 'text-slate-600';

    return (
        <div className="bg-white rounded-xl border border-slate-200 p-4">
            <div className="flex items-center gap-3">
                <div className={cn('p-2 rounded-lg', bgColor)}>
                    <Icon className={cn('h-5 w-5', textColor)} />
                </div>
                <div>
                    <p className="text-sm text-slate-500">{label}</p>
                    <p className="text-2xl font-bold text-slate-900">{value}</p>
                </div>
            </div>
        </div>
    );
}

function JobStatusBadge({ status }: { status: string }) {
    const styles = {
        published: 'bg-emerald-100 text-emerald-700',
        draft: 'bg-slate-100 text-slate-600',
        closed: 'bg-red-100 text-red-700',
    }[status] || 'bg-slate-100 text-slate-600';

    return (
        <span className={cn('px-2 py-1 text-xs font-medium rounded-full capitalize', styles)}>
            {status}
        </span>
    );
}

function AiScoreBadge({ score, recommendation }: { score: number; recommendation: string }) {
    const colors = {
        strong_yes: 'from-emerald-500 to-teal-500',
        yes: 'from-blue-500 to-indigo-500',
        maybe: 'from-amber-500 to-orange-500',
        no: 'from-red-500 to-pink-500',
    }[recommendation] || 'from-slate-500 to-slate-600';

    const icons = {
        strong_yes: ThumbsUp,
        yes: ThumbsUp,
        maybe: Clock,
        no: ThumbsDown,
    };

    const Icon = icons[recommendation as keyof typeof icons] || Clock;

    return (
        <div className={cn('flex items-center gap-1.5 px-2 py-1 rounded-full bg-gradient-to-r text-white', colors)}>
            <Icon className="h-3 w-3" />
            <span className="text-xs font-semibold">{score}%</span>
        </div>
    );
}

function StageBadge({ stage }: { stage: string }) {
    const styles = {
        received: 'bg-slate-100 text-slate-600',
        screening: 'bg-blue-100 text-blue-700',
        interview: 'bg-purple-100 text-purple-700',
        offer: 'bg-amber-100 text-amber-700',
        hired: 'bg-emerald-100 text-emerald-700',
        rejected: 'bg-red-100 text-red-700',
    }[stage] || 'bg-slate-100 text-slate-600';

    return (
        <span className={cn('px-2 py-1 text-xs font-medium rounded-full capitalize', styles)}>
            {stage}
        </span>
    );
}
