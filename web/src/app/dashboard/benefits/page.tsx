'use client';

import { Header } from '@/components/layout/Header';
import { formatNaira, formatDate } from '@/lib/nigerian-locale';
import { cn } from '@/lib/utils';
import {
    Heart, Plus, Shield, Users, FileText,
    CheckCircle, Clock, AlertCircle, ChevronRight
} from 'lucide-react';

// Nigerian HMO Providers
const hmoProviders = [
    { name: 'Hygeia HMO', logo: '🏥', planCount: 3 },
    { name: 'Leadway Health', logo: '💊', planCount: 2 },
    { name: 'Reliance HMO', logo: '🩺', planCount: 4 },
];

const myBenefits = [
    {
        id: '1',
        type: 'HMO',
        name: 'Family Health Plan',
        provider: 'Hygeia HMO',
        coverage: 'Employee + 4 Dependents',
        status: 'active',
        monthlyPremium: 45000,
        employerContribution: 35000,
        employeeContribution: 10000,
        renewalDate: '2024-12-31',
    },
    {
        id: '2',
        type: 'Pension AVC',
        name: 'Additional Voluntary Contribution',
        provider: 'ARM Pension',
        coverage: '5% of Basic Salary',
        status: 'active',
        monthlyPremium: 25000,
        employerContribution: 0,
        employeeContribution: 25000,
        renewalDate: null,
    },
];

const pendingClaims = [
    { id: '1', type: 'Medical', description: 'Hospital consultation', amount: 25000, status: 'pending', submittedAt: '2024-01-28' },
    { id: '2', type: 'Optical', description: 'Eye glasses', amount: 45000, status: 'approved', submittedAt: '2024-01-20' },
];

const dependents = [
    { name: 'Amina Ogunsakin', relationship: 'Spouse', covered: true },
    { name: 'Tunde Ogunsakin', relationship: 'Child', covered: true },
    { name: 'Kemi Ogunsakin', relationship: 'Child', covered: true },
];

export default function BenefitsPage() {
    return (
        <div className="min-h-screen">
            <Header
                title="Benefits"
                subtitle="HMO, Pension & Wellness"
            />

            <div className="p-6 space-y-6">
                {/* Summary */}
                <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <StatCard icon={Heart} label="Active Plans" value="2" color="rose" />
                    <StatCard icon={Users} label="Covered Dependents" value="3" color="blue" />
                    <StatCard icon={FileText} label="Pending Claims" value="1" color="amber" />
                    <StatCard icon={Shield} label="Total Coverage" value={formatNaira(5000000, false)} color="emerald" />
                </div>

                {/* My Benefits */}
                <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                    <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                        <h3 className="text-lg font-semibold text-slate-900">My Benefits</h3>
                        <button className="flex items-center gap-1 px-3 py-1.5 bg-gradient-to-r from-rose-600 to-pink-600 text-white rounded-lg text-sm font-medium hover:from-rose-700 hover:to-pink-700">
                            <Plus className="h-4 w-4" />
                            Enroll
                        </button>
                    </div>
                    <div className="divide-y divide-slate-100">
                        {myBenefits.map((benefit) => (
                            <div key={benefit.id} className="px-6 py-4 hover:bg-slate-50 transition-colors">
                                <div className="flex items-start justify-between mb-3">
                                    <div>
                                        <div className="flex items-center gap-2">
                                            <span className="px-2 py-0.5 bg-rose-100 text-rose-700 text-xs font-medium rounded">
                                                {benefit.type}
                                            </span>
                                            <p className="font-medium text-slate-900">{benefit.name}</p>
                                        </div>
                                        <p className="text-sm text-slate-500 mt-1">{benefit.provider}</p>
                                    </div>
                                    <StatusBadge status={benefit.status} />
                                </div>
                                <div className="grid grid-cols-3 gap-4 mt-3 pt-3 border-t border-slate-100">
                                    <div>
                                        <p className="text-xs text-slate-500">Coverage</p>
                                        <p className="text-sm font-medium text-slate-900">{benefit.coverage}</p>
                                    </div>
                                    <div>
                                        <p className="text-xs text-slate-500">Your Contribution</p>
                                        <p className="text-sm font-medium text-slate-900">{formatNaira(benefit.employeeContribution)}/mo</p>
                                    </div>
                                    <div>
                                        <p className="text-xs text-slate-500">Employer Pays</p>
                                        <p className="text-sm font-medium text-emerald-600">{formatNaira(benefit.employerContribution)}/mo</p>
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    {/* Dependents */}
                    <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                            <h3 className="text-lg font-semibold text-slate-900">My Dependents</h3>
                            <button className="text-sm font-medium text-rose-600 hover:text-rose-700">
                                + Add Dependent
                            </button>
                        </div>
                        <div className="divide-y divide-slate-100">
                            {dependents.map((dep) => (
                                <div key={dep.name} className="px-6 py-3 flex items-center justify-between">
                                    <div className="flex items-center gap-3">
                                        <div className="w-8 h-8 rounded-full bg-rose-100 flex items-center justify-center">
                                            <Users className="h-4 w-4 text-rose-600" />
                                        </div>
                                        <div>
                                            <p className="font-medium text-slate-900">{dep.name}</p>
                                            <p className="text-sm text-slate-500">{dep.relationship}</p>
                                        </div>
                                    </div>
                                    {dep.covered ? (
                                        <CheckCircle className="h-5 w-5 text-emerald-500" />
                                    ) : (
                                        <AlertCircle className="h-5 w-5 text-slate-300" />
                                    )}
                                </div>
                            ))}
                        </div>
                    </div>

                    {/* Claims */}
                    <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
                        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
                            <h3 className="text-lg font-semibold text-slate-900">Recent Claims</h3>
                            <button className="flex items-center gap-1 text-sm font-medium text-rose-600 hover:text-rose-700">
                                <Plus className="h-4 w-4" />
                                Submit Claim
                            </button>
                        </div>
                        <div className="divide-y divide-slate-100">
                            {pendingClaims.map((claim) => (
                                <div key={claim.id} className="px-6 py-3 flex items-center justify-between">
                                    <div>
                                        <p className="font-medium text-slate-900">{claim.description}</p>
                                        <p className="text-sm text-slate-500">{claim.type} • {formatDate(claim.submittedAt)}</p>
                                    </div>
                                    <div className="text-right flex items-center gap-3">
                                        <div>
                                            <p className="font-semibold text-slate-900">{formatNaira(claim.amount)}</p>
                                            <ClaimStatusBadge status={claim.status} />
                                        </div>
                                        <ChevronRight className="h-5 w-5 text-slate-400" />
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>

                {/* HMO Providers */}
                <div className="bg-white rounded-xl border border-slate-200 p-6">
                    <h3 className="text-lg font-semibold text-slate-900 mb-4">Nigerian HMO Providers</h3>
                    <div className="grid grid-cols-3 gap-4">
                        {hmoProviders.map((provider) => (
                            <div key={provider.name} className="p-4 bg-slate-50 rounded-lg text-center hover:bg-slate-100 transition-colors cursor-pointer">
                                <div className="text-3xl mb-2">{provider.logo}</div>
                                <p className="font-medium text-slate-900">{provider.name}</p>
                                <p className="text-sm text-slate-500">{provider.planCount} plans</p>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

function StatCard({ icon: Icon, label, value, color }: { icon: any; label: string; value: string; color: string }) {
    const bgColor = {
        rose: 'bg-rose-100',
        blue: 'bg-blue-100',
        amber: 'bg-amber-100',
        emerald: 'bg-emerald-100',
    }[color] || 'bg-slate-100';

    const textColor = {
        rose: 'text-rose-600',
        blue: 'text-blue-600',
        amber: 'text-amber-600',
        emerald: 'text-emerald-600',
    }[color] || 'text-slate-600';

    return (
        <div className="bg-white rounded-xl border border-slate-200 p-4">
            <div className="flex items-center gap-3">
                <div className={cn('p-2 rounded-lg', bgColor)}>
                    <Icon className={cn('h-5 w-5', textColor)} />
                </div>
                <div>
                    <p className="text-sm text-slate-500">{label}</p>
                    <p className="text-xl font-bold text-slate-900">{value}</p>
                </div>
            </div>
        </div>
    );
}

function StatusBadge({ status }: { status: string }) {
    return (
        <span className={cn(
            'px-2 py-1 text-xs font-medium rounded-full',
            status === 'active' && 'bg-emerald-100 text-emerald-700'
        )}>
            Active
        </span>
    );
}

function ClaimStatusBadge({ status }: { status: string }) {
    const styles = {
        pending: 'text-amber-600',
        approved: 'text-emerald-600',
        rejected: 'text-red-600',
        paid: 'text-blue-600',
    }[status] || 'text-slate-600';

    return (
        <span className={cn('text-xs font-medium capitalize', styles)}>
            {status}
        </span>
    );
}
