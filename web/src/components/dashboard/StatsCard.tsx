import { cn } from '@/lib/utils';
import { LucideIcon, TrendingUp, TrendingDown } from 'lucide-react';
import { formatNaira, formatNairaShort, formatPercentChange } from '@/lib/nigerian-locale';

interface StatsCardProps {
    title: string;
    value: string | number;
    change?: number;
    changeLabel?: string;
    icon: LucideIcon;
    iconColor?: string;
    isCurrency?: boolean;
}

export function StatsCard({
    title,
    value,
    change,
    changeLabel,
    icon: Icon,
    iconColor = 'text-emerald-500',
    isCurrency = false,
}: StatsCardProps) {
    const displayValue = isCurrency && typeof value === 'number'
        ? formatNairaShort(value)
        : value;

    const changeInfo = change !== undefined ? formatPercentChange(change) : null;

    return (
        <div className="bg-white rounded-xl border border-slate-200 p-6 hover:shadow-lg transition-shadow duration-300">
            <div className="flex items-start justify-between">
                <div className="flex-1">
                    <p className="text-sm font-medium text-slate-500">{title}</p>
                    <p className="mt-2 text-3xl font-bold text-slate-900">{displayValue}</p>

                    {changeInfo && (
                        <div className="mt-2 flex items-center gap-1">
                            {changeInfo.isPositive ? (
                                <TrendingUp className="h-4 w-4 text-emerald-500" />
                            ) : (
                                <TrendingDown className="h-4 w-4 text-red-500" />
                            )}
                            <span className={cn(
                                'text-sm font-medium',
                                changeInfo.isPositive ? 'text-emerald-600' : 'text-red-600'
                            )}>
                                {changeInfo.text}
                            </span>
                            {changeLabel && (
                                <span className="text-sm text-slate-400">{changeLabel}</span>
                            )}
                        </div>
                    )}
                </div>

                <div className={cn(
                    'p-3 rounded-xl bg-gradient-to-br',
                    iconColor === 'text-emerald-500' && 'from-emerald-50 to-teal-50',
                    iconColor === 'text-blue-500' && 'from-blue-50 to-indigo-50',
                    iconColor === 'text-purple-500' && 'from-purple-50 to-pink-50',
                    iconColor === 'text-orange-500' && 'from-orange-50 to-amber-50',
                )}>
                    <Icon className={cn('h-6 w-6', iconColor)} />
                </div>
            </div>
        </div>
    );
}
