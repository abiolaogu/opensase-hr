'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { cn } from '@/lib/utils';
import {
    Users,
    Banknote,
    Calendar,
    Target,
    Briefcase,
    Heart,
    Shield,
    LayoutDashboard,
    Settings,
    LogOut,
} from 'lucide-react';

const navigation = [
    { name: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { name: 'Employees', href: '/dashboard/employees', icon: Users },
    { name: 'Payroll', href: '/dashboard/payroll', icon: Banknote },
    { name: 'Leave', href: '/dashboard/leave', icon: Calendar },
    { name: 'Performance', href: '/dashboard/performance', icon: Target },
    { name: 'Recruitment', href: '/dashboard/recruitment', icon: Briefcase },
    { name: 'Benefits', href: '/dashboard/benefits', icon: Heart },
    { name: 'Compliance', href: '/dashboard/compliance', icon: Shield },
];

const bottomNavigation = [
    { name: 'Settings', href: '/dashboard/settings', icon: Settings },
];

export function Sidebar() {
    const pathname = usePathname();

    return (
        <div className="flex h-full w-64 flex-col bg-gradient-to-b from-slate-900 to-slate-800 text-white">
            {/* Logo */}
            <div className="flex h-16 items-center gap-2 px-6 border-b border-slate-700">
                <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-emerald-400 to-teal-500 flex items-center justify-center font-bold text-slate-900">
                    HR
                </div>
                <span className="text-lg font-semibold">OpenSASE HR</span>
            </div>

            {/* Navigation */}
            <nav className="flex-1 px-3 py-4 space-y-1 overflow-y-auto">
                {navigation.map((item) => {
                    const isActive = pathname === item.href || pathname.startsWith(item.href + '/');
                    return (
                        <Link
                            key={item.name}
                            href={item.href}
                            className={cn(
                                'flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all duration-200',
                                isActive
                                    ? 'bg-gradient-to-r from-emerald-500/20 to-teal-500/20 text-emerald-400 border border-emerald-500/30'
                                    : 'text-slate-400 hover:text-white hover:bg-slate-700/50'
                            )}
                        >
                            <item.icon className="h-5 w-5" />
                            {item.name}
                            {isActive && (
                                <div className="ml-auto w-1.5 h-1.5 rounded-full bg-emerald-400" />
                            )}
                        </Link>
                    );
                })}
            </nav>

            {/* Bottom Navigation */}
            <div className="px-3 py-4 border-t border-slate-700 space-y-1">
                {bottomNavigation.map((item) => (
                    <Link
                        key={item.name}
                        href={item.href}
                        className="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium text-slate-400 hover:text-white hover:bg-slate-700/50 transition-colors"
                    >
                        <item.icon className="h-5 w-5" />
                        {item.name}
                    </Link>
                ))}
                <button className="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium text-slate-400 hover:text-red-400 hover:bg-red-500/10 transition-colors">
                    <LogOut className="h-5 w-5" />
                    Logout
                </button>
            </div>

            {/* User Info */}
            <div className="px-3 py-4 border-t border-slate-700">
                <div className="flex items-center gap-3 px-3 py-2">
                    <div className="w-10 h-10 rounded-full bg-gradient-to-br from-emerald-400 to-teal-500 flex items-center justify-center font-semibold text-slate-900">
                        AO
                    </div>
                    <div className="flex-1 min-w-0">
                        <p className="text-sm font-medium text-white truncate">Abiola Ogunsakin</p>
                        <p className="text-xs text-slate-400 truncate">HR Manager</p>
                    </div>
                </div>
            </div>
        </div>
    );
}
