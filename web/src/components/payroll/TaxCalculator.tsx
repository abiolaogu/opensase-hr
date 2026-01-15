'use client';

import { useState } from 'react';
import { formatNaira } from '@/lib/nigerian-locale';
import { Calculator, Info } from 'lucide-react';

export function TaxCalculator() {
    const [salary, setSalary] = useState<string>('');
    const [result, setResult] = useState<{
        gross: number;
        paye: number;
        pension: number;
        nhf: number;
        net: number;
    } | null>(null);

    const calculateTax = () => {
        const grossMonthly = parseFloat(salary) || 0;
        if (grossMonthly <= 0) return;

        // Simplified calculation (matches Rust backend logic)
        const basicRatio = 0.60;
        const housingRatio = 0.25;
        const transportRatio = 0.15;

        const basic = grossMonthly * basicRatio;
        const housing = grossMonthly * housingRatio;
        const transport = grossMonthly * transportRatio;

        // Pension: 8% of (Basic + Housing + Transport)
        const pensionable = basic + housing + transport;
        const pension = pensionable * 0.08;

        // NHF: 2.5% of Basic
        const nhf = basic * 0.025;

        // Simplified PAYE calculation (annual then monthly)
        const grossAnnual = grossMonthly * 12;
        const pensionAnnual = pension * 12;
        const nhfAnnual = nhf * 12;

        // CRA: 20% of gross + higher of (200k or 1% of gross)
        const craPercentage = grossAnnual * 0.20;
        const craMin = Math.max(200000, grossAnnual * 0.01);
        const totalCra = craPercentage + craMin;

        // Taxable income
        const taxableAnnual = Math.max(0, grossAnnual - totalCra - pensionAnnual - nhfAnnual);

        // Progressive tax
        let tax = 0;
        let remaining = taxableAnnual;
        const bands = [
            { threshold: 300000, rate: 0.07 },
            { threshold: 300000, rate: 0.11 },
            { threshold: 500000, rate: 0.15 },
            { threshold: 500000, rate: 0.19 },
            { threshold: 1600000, rate: 0.21 },
            { threshold: Infinity, rate: 0.24 },
        ];

        for (const band of bands) {
            if (remaining <= 0) break;
            const taxable = Math.min(remaining, band.threshold);
            tax += taxable * band.rate;
            remaining -= taxable;
        }

        const payeMonthly = tax / 12;
        const net = grossMonthly - payeMonthly - pension - nhf;

        setResult({
            gross: grossMonthly,
            paye: payeMonthly,
            pension,
            nhf,
            net,
        });
    };

    return (
        <div className="bg-white rounded-xl border border-slate-200 overflow-hidden">
            <div className="px-6 py-4 border-b border-slate-100 flex items-center gap-2">
                <Calculator className="h-5 w-5 text-emerald-600" />
                <h3 className="text-lg font-semibold text-slate-900">Tax Calculator</h3>
            </div>

            <div className="p-6">
                <div className="mb-4">
                    <label className="block text-sm font-medium text-slate-700 mb-1">
                        Monthly Gross Salary (₦)
                    </label>
                    <div className="flex gap-2">
                        <input
                            type="number"
                            value={salary}
                            onChange={(e) => setSalary(e.target.value)}
                            placeholder="e.g., 500000"
                            className="flex-1 px-4 py-2.5 border border-slate-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                        />
                        <button
                            onClick={calculateTax}
                            className="px-6 py-2.5 bg-gradient-to-r from-emerald-600 to-teal-600 text-white font-medium rounded-lg hover:from-emerald-700 hover:to-teal-700 transition-colors"
                        >
                            Calculate
                        </button>
                    </div>
                </div>

                {result && (
                    <div className="mt-6 p-4 bg-slate-50 rounded-lg space-y-3">
                        <div className="flex justify-between">
                            <span className="text-slate-600">Gross Salary</span>
                            <span className="font-medium text-slate-900">{formatNaira(result.gross)}</span>
                        </div>
                        <hr className="border-slate-200" />
                        <div className="flex justify-between text-sm">
                            <span className="text-slate-500">PAYE Tax</span>
                            <span className="text-red-600">-{formatNaira(result.paye)}</span>
                        </div>
                        <div className="flex justify-between text-sm">
                            <span className="text-slate-500">Pension (8%)</span>
                            <span className="text-red-600">-{formatNaira(result.pension)}</span>
                        </div>
                        <div className="flex justify-between text-sm">
                            <span className="text-slate-500">NHF (2.5%)</span>
                            <span className="text-red-600">-{formatNaira(result.nhf)}</span>
                        </div>
                        <hr className="border-slate-200" />
                        <div className="flex justify-between text-lg font-bold">
                            <span className="text-slate-900">Net Salary</span>
                            <span className="text-emerald-600">{formatNaira(result.net)}</span>
                        </div>
                    </div>
                )}

                <div className="mt-4 flex items-start gap-2 text-sm text-slate-500">
                    <Info className="h-4 w-4 mt-0.5 flex-shrink-0" />
                    <p>
                        This calculator uses 2024 Nigerian PAYE tax bands with CRA deduction.
                        Assumes 60% basic, 25% housing, 15% transport.
                    </p>
                </div>
            </div>
        </div>
    );
}
