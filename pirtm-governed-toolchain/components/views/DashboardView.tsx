'use client';

import React, { useState, useEffect, useMemo } from 'react';
import {
  Activity,
  ShieldCheck,
  AlertTriangle,
  Flame,
  Radio,
  RefreshCw,
  Filter,
  CheckCircle2,
  Lock,
  Unlock,
  AlertCircle,
  FileCheck,
  Search,
  ExternalLink,
  ChevronRight,
  TrendingDown,
  TrendingUp,
  Cpu,
  Clock
} from 'lucide-react';
import { ResponsiveContainer, LineChart, Line, XAxis, YAxis, Tooltip, CartesianGrid, ReferenceLine } from 'recharts';
import { INITIAL_AUDIT_EVENTS, AuditEvent } from '@/lib/pirtm-data';

interface ContractSession {
  id: string;
  client: string;
  type: string;
  uptime: string;
  rhoCurrent: number;
  driftPercent: number;
  status: 'NOMINAL' | 'MONITORED' | 'QUARANTINED';
  receiptCount: number;
}

export function DashboardView() {
  const [auditEvents, setAuditEvents] = useState<AuditEvent[]>(INITIAL_AUDIT_EVENTS);
  const [severityFilter, setSeverityFilter] = useState<'all' | 'info' | 'warning' | 'violation'>('all');
  const [searchQuery, setSearchQuery] = useState('');
  const [isLiveStreaming, setIsLiveStreaming] = useState(true);
  const [killSwitchArmed, setKillSwitchArmed] = useState(true);
  const [manualTrip, setManualTrip] = useState(false);
  const [selectedEvent, setSelectedEvent] = useState<AuditEvent | null>(null);

  // Time series data for Spectral Radius
  const [spectralHistory, setSpectralHistory] = useState([
    { time: '15:35', rho: 0.38, warn: 0.85, halt: 1.0 },
    { time: '15:36', rho: 0.42, warn: 0.85, halt: 1.0 },
    { time: '15:37', rho: 0.40, warn: 0.85, halt: 1.0 },
    { time: '15:38', rho: 0.88, warn: 0.85, halt: 1.0 },
    { time: '15:39', rho: 0.65, warn: 0.85, halt: 1.0 },
    { time: '15:40', rho: manualTrip ? 1.15 : 0.41, warn: 0.85, halt: 1.0 },
    { time: '15:41', rho: manualTrip ? 1.15 : 0.39, warn: 0.85, halt: 1.0 },
    { time: '15:42', rho: manualTrip ? 1.15 : 0.42, warn: 0.85, halt: 1.0 },
  ]);

  const [activeSessions, setActiveSessions] = useState<ContractSession[]>([
    {
      id: 'sess-ai-agent-alpha-84',
      client: 'Claude-3.7-Governed-Agent',
      type: 'MOC Loop Policy',
      uptime: '42m 10s',
      rhoCurrent: 0.412,
      driftPercent: 0.12,
      status: 'NOMINAL',
      receiptCount: 1420
    },
    {
      id: 'sess-matrix-polyhedral-12',
      client: 'MLIR Affine Compiler Node',
      type: 'Tensor Reduction',
      uptime: '1h 14m',
      rhoCurrent: 0.284,
      driftPercent: 0.04,
      status: 'NOMINAL',
      receiptCount: 9410
    },
    {
      id: 'sess-agent-unconstrained-09',
      client: 'Untrusted Tool Call Service',
      type: 'Recursive Feedback',
      uptime: '5m 22s',
      rhoCurrent: 1.142,
      driftPercent: 14.2,
      status: 'QUARANTINED',
      receiptCount: 18
    },
    {
      id: 'sess-robotics-kinematics-04',
      client: 'Physical Controller RT',
      type: 'Phase Manifold Sim',
      uptime: '3h 48m',
      rhoCurrent: 0.582,
      driftPercent: 0.80,
      status: 'NOMINAL',
      receiptCount: 38412
    }
  ]);

  // Periodic simulated live updates
  useEffect(() => {
    if (!isLiveStreaming) return;
    const interval = setInterval(() => {
      const now = new Date();
      const timeStr = `${now.getHours()}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`;
      
      const newRho = manualTrip ? 1.142 : +(0.35 + Math.random() * 0.15).toFixed(3);
      
      setSpectralHistory((prev) => {
        const next = [...prev.slice(1), { time: timeStr, rho: newRho, warn: 0.85, halt: 1.0 }];
        return next;
      });
    }, 4000);

    return () => clearInterval(interval);
  }, [isLiveStreaming, manualTrip]);

  const filteredEvents = useMemo(() => {
    return auditEvents.filter((evt) => {
      if (severityFilter !== 'all' && evt.severity !== severityFilter) return false;
      if (searchQuery) {
        const q = searchQuery.toLowerCase();
        return (
          evt.receiptHash.toLowerCase().includes(q) ||
          evt.sessionOrArtifact.toLowerCase().includes(q) ||
          evt.details.toLowerCase().includes(q)
        );
      }
      return true;
    });
  }, [auditEvents, severityFilter, searchQuery]);

  const handleToggleManualTrip = () => {
    setManualTrip(!manualTrip);
    if (!manualTrip) {
      const tripEvt: AuditEvent = {
        id: `evt-${Date.now()}`,
        timestamp: new Date().toLocaleTimeString(),
        eventType: 'WARD_TRIP',
        severity: 'violation',
        sessionOrArtifact: 'manual-test-circuit-breaker',
        receiptHash: '0xHALT-MANUAL-TRIGGER-TRIP',
        spectralRadius: 1.18,
        driftPercent: 18.0,
        details: 'Manual Circuit Breaker Engaged: Immediate emergency isolation triggered by operator.',
        status: 'HALTED'
      };
      setAuditEvents((prev) => [tripEvt, ...prev]);
    }
  };

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">
      {/* Top Header & Telemetry Status */}
      <div className="flex flex-col md:flex-row md:items-center justify-between pb-6 border-b border-[#233147] gap-4">
        <div>
          <div className="flex items-center gap-2 text-xs font-mono text-cyan-400 mb-1">
            <Activity className="w-4 h-4" />
            <span>REAL-TIME GOVERNANCE TELEMETRY</span>
          </div>
          <h1 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
            Spectral Radius &amp; WardMonitor Dashboard
          </h1>
          <p className="text-xs sm:text-sm text-[#8b949e] mt-1">
            Continuous Lyapunov contractivity verification, runtime drift tracking, and kill-switch state.
          </p>
        </div>

        {/* Live Controls */}
        <div className="flex items-center gap-3">
          <button
            onClick={() => setIsLiveStreaming(!isLiveStreaming)}
            className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-mono font-medium border transition-colors cursor-pointer ${
              isLiveStreaming
                ? 'bg-emerald-950/60 border-emerald-800/60 text-emerald-300'
                : 'bg-[#141e2e] border-[#273852] text-[#8b949e]'
            }`}
          >
            <Radio className={`w-3.5 h-3.5 ${isLiveStreaming ? 'animate-pulse text-emerald-400' : ''}`} />
            <span>{isLiveStreaming ? 'Live Stream: Active' : 'Stream Paused'}</span>
          </button>

          <button
            onClick={handleToggleManualTrip}
            className={`flex items-center gap-1.5 px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer ${
              manualTrip
                ? 'bg-rose-600 hover:bg-rose-500 text-white shadow-lg shadow-rose-950/60'
                : 'bg-rose-950/40 hover:bg-rose-900/50 border border-rose-800/50 text-rose-300'
            }`}
          >
            <AlertTriangle className="w-3.5 h-3.5" />
            <span>{manualTrip ? 'Clear Emergency Trip' : 'Test Circuit Breaker'}</span>
          </button>
        </div>
      </div>

      {/* Top 4 KPI Metrics */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Metric 1 */}
        <div className="p-5 rounded-xl bg-[#0e1624] border border-[#202e42] flex flex-col justify-between">
          <div className="flex items-center justify-between text-[#8b949e] text-xs">
            <span>Effective Spectral Radius (ρ)</span>
            <Activity className="w-4 h-4 text-cyan-400" />
          </div>
          <div className="my-2">
            <div className={`text-2xl font-bold font-mono ${manualTrip ? 'text-rose-400' : 'text-emerald-400'}`}>
              {manualTrip ? '1.142 (BREACH)' : '0.412'}
            </div>
            <span className="text-[11px] text-[#6e7681] font-mono">Boundary: ρ &lt; 1.000</span>
          </div>
          <div className="text-[11px] font-mono text-emerald-400 flex items-center gap-1">
            <TrendingDown className="w-3.5 h-3.5" />
            <span>Lyapunov Energy Stable</span>
          </div>
        </div>

        {/* Metric 2 */}
        <div className="p-5 rounded-xl bg-[#0e1624] border border-[#202e42] flex flex-col justify-between">
          <div className="flex items-center justify-between text-[#8b949e] text-xs">
            <span>WardMonitor State</span>
            <ShieldCheck className="w-4 h-4 text-emerald-400" />
          </div>
          <div className="my-2">
            <div className={`text-2xl font-bold font-mono ${manualTrip ? 'text-rose-400' : 'text-emerald-400'}`}>
              {manualTrip ? 'TRIPPED & HALTED' : 'ARMED & NOMINAL'}
            </div>
            <span className="text-[11px] text-[#6e7681] font-mono">Threshold: ρ_warn = 0.85</span>
          </div>
          <div className="text-[11px] font-mono text-cyan-400 flex items-center gap-1">
            <Lock className="w-3.5 h-3.5" />
            <span>Continuous Invariant Latch</span>
          </div>
        </div>

        {/* Metric 3 */}
        <div className="p-5 rounded-xl bg-[#0e1624] border border-[#202e42] flex flex-col justify-between">
          <div className="flex items-center justify-between text-[#8b949e] text-xs">
            <span>Active Governed Sessions</span>
            <Cpu className="w-4 h-4 text-sky-400" />
          </div>
          <div className="my-2">
            <div className="text-2xl font-bold font-mono text-[#e6edf3]">
              {activeSessions.length} Nodes
            </div>
            <span className="text-[11px] text-[#6e7681] font-mono">1 Quarantined</span>
          </div>
          <div className="text-[11px] font-mono text-sky-400 flex items-center gap-1">
            <Clock className="w-3.5 h-3.5" />
            <span>Avg Uptime: 1h 26m</span>
          </div>
        </div>

        {/* Metric 4 */}
        <div className="p-5 rounded-xl bg-[#0e1624] border border-[#202e42] flex flex-col justify-between">
          <div className="flex items-center justify-between text-[#8b949e] text-xs">
            <span>Blake3 Merkle Receipts</span>
            <FileCheck className="w-4 h-4 text-amber-400" />
          </div>
          <div className="my-2">
            <div className="text-2xl font-bold font-mono text-amber-300">
              49,262 Total
            </div>
            <span className="text-[11px] text-[#6e7681] font-mono">0 Forgeries Detected</span>
          </div>
          <div className="text-[11px] font-mono text-emerald-400 flex items-center gap-1">
            <CheckCircle2 className="w-3.5 h-3.5" />
            <span>100% Provenance Audit</span>
          </div>
        </div>
      </div>

      {/* Main Row: Spectral Radius Time Series Chart & WardMonitor Watchdog */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Left: Recharts Line Chart */}
        <div className="lg:col-span-8 p-6 rounded-xl bg-[#0e1624] border border-[#202e42] flex flex-col justify-between">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between pb-4 mb-4 border-b border-[#1e2a3c] gap-2">
            <div>
              <h2 className="text-base font-bold text-[#e6edf3]">
                Spectral Radius Timeline (ρ over session)
              </h2>
              <p className="text-xs text-[#8b949e]">
                Continuous small-gain measurement. Value must remain strictly below the red halt line (1.000).
              </p>
            </div>
            <div className="flex items-center gap-3 text-xs font-mono">
              <span className="flex items-center gap-1 text-emerald-400">
                <span className="w-2.5 h-1 bg-emerald-400 rounded" /> ρ(A) Measured
              </span>
              <span className="flex items-center gap-1 text-amber-400">
                <span className="w-2.5 h-1 bg-amber-400 rounded" /> ρ_warn (0.85)
              </span>
              <span className="flex items-center gap-1 text-rose-500">
                <span className="w-2.5 h-1 bg-rose-500 rounded" /> ρ_halt (1.00)
              </span>
            </div>
          </div>

          {/* Chart Container */}
          <div className="h-64 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={spectralHistory} margin={{ top: 10, right: 20, left: -20, bottom: 0 }}>
                <CartesianGrid strokeDasharray="3 3" stroke="#1b2738" vertical={false} />
                <XAxis dataKey="time" stroke="#6e7681" tick={{ fontSize: 10, fill: '#8b949e' }} />
                <YAxis domain={[0, 1.4]} stroke="#6e7681" tick={{ fontSize: 10, fill: '#8b949e' }} />
                <Tooltip
                  contentStyle={{ backgroundColor: '#090d14', borderColor: '#233147', borderRadius: '8px', fontSize: '11px', fontFamily: 'monospace' }}
                  labelStyle={{ color: '#e6edf3' }}
                />
                <ReferenceLine y={0.85} stroke="#f59e0b" strokeDasharray="3 3" label={{ value: 'Warn: 0.85', fill: '#f59e0b', fontSize: 10, position: 'right' }} />
                <ReferenceLine y={1.0} stroke="#f43f5e" strokeDasharray="2 2" strokeWidth={1.5} label={{ value: 'HALT: 1.0', fill: '#f43f5e', fontSize: 10, position: 'right' }} />
                <Line
                  type="monotone"
                  dataKey="rho"
                  stroke={manualTrip ? '#f43f5e' : '#10b981'}
                  strokeWidth={2.5}
                  dot={{ r: 3, fill: manualTrip ? '#f43f5e' : '#10b981' }}
                  isAnimationActive={false}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Right: WardMonitor Control & Drift Watchdog */}
        <div className="lg:col-span-4 p-6 rounded-xl bg-[#0e1624] border border-[#202e42] flex flex-col justify-between space-y-4">
          <div>
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-bold text-[#e6edf3] text-sm flex items-center gap-2">
                <ShieldCheck className="w-4 h-4 text-cyan-400" />
                WardMonitor Watchdog
              </h3>
              <span className={`px-2 py-0.5 rounded text-[10px] font-mono font-bold ${
                manualTrip ? 'bg-rose-950 text-rose-300 border border-rose-800' : 'bg-emerald-950 text-emerald-300 border border-emerald-800'
              }`}>
                {manualTrip ? 'TRIP LATCHED' : 'ARMED'}
              </span>
            </div>

            <p className="text-xs text-[#8b949e] leading-relaxed mb-4">
              Real-time hardware &amp; software gate preventing runaway recursive token accumulation.
            </p>

            <div className="space-y-3 font-mono text-xs">
              <div className="p-3 rounded-lg bg-[#090d14] border border-[#1e2a3c] flex items-center justify-between">
                <span className="text-[#8b949e]">Current Phase Drift:</span>
                <span className={manualTrip ? 'text-rose-400 font-bold' : 'text-emerald-400 font-bold'}>
                  {manualTrip ? '+14.2% (UNSAFE)' : '0.04% (NOMINAL)'}
                </span>
              </div>

              <div className="p-3 rounded-lg bg-[#090d14] border border-[#1e2a3c] flex items-center justify-between">
                <span className="text-[#8b949e]">Contractivity Gate:</span>
                <span className="text-cyan-300 font-semibold">Lean 4 Certified</span>
              </div>

              <div className="p-3 rounded-lg bg-[#090d14] border border-[#1e2a3c] flex items-center justify-between">
                <span className="text-[#8b949e]">Kill-Switch Arming:</span>
                <span className="text-emerald-400 font-bold">LATCH_ENABLED</span>
              </div>
            </div>
          </div>

          <div className="p-3 rounded-lg bg-emerald-950/40 border border-emerald-800/40 text-[11px] text-[#cbd5e1] flex items-center gap-2">
            <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
            <span>Lyapunov energy strictly bounded by Theorem 1.</span>
          </div>
        </div>
      </div>

      {/* Active Sessions Table */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-base font-bold text-[#e6edf3]">
            Active Governed Contracts &amp; Sessions
          </h2>
          <span className="text-xs text-[#8b949e] font-mono">
            {activeSessions.length} Active Endpoints
          </span>
        </div>

        <div className="border border-[#233147] rounded-xl bg-[#0e1624] overflow-x-auto">
          <table className="w-full text-left text-xs">
            <thead className="bg-[#090d14] text-[#8b949e] border-b border-[#233147] font-mono text-[11px]">
              <tr>
                <th className="py-3 px-4">Session ID</th>
                <th className="py-3 px-4">Client / Agent</th>
                <th className="py-3 px-4">Operator Type</th>
                <th className="py-3 px-4">Uptime</th>
                <th className="py-3 px-4">Current ρ</th>
                <th className="py-3 px-4">Status</th>
                <th className="py-3 px-4 text-right">Receipts</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-[#1e2c40]">
              {activeSessions.map((sess) => (
                <tr key={sess.id} className="hover:bg-[#121c2d] transition-colors font-mono">
                  <td className="py-3.5 px-4 font-bold text-cyan-400 whitespace-nowrap">
                    {sess.id}
                  </td>
                  <td className="py-3.5 px-4 text-[#e6edf3] font-sans">
                    {sess.client}
                  </td>
                  <td className="py-3.5 px-4 text-[#8b949e]">
                    {sess.type}
                  </td>
                  <td className="py-3.5 px-4 text-[#8b949e] whitespace-nowrap">
                    {sess.uptime}
                  </td>
                  <td className="py-3.5 px-4 whitespace-nowrap font-bold">
                    <span className={sess.rhoCurrent < 1.0 ? 'text-emerald-400' : 'text-rose-400'}>
                      {sess.rhoCurrent.toFixed(3)}
                    </span>
                  </td>
                  <td className="py-3.5 px-4 whitespace-nowrap">
                    <span className={`px-2 py-0.5 rounded text-[10px] font-bold border ${
                      sess.status === 'NOMINAL'
                        ? 'bg-emerald-950/80 text-emerald-300 border-emerald-800/60'
                        : 'bg-rose-950/80 text-rose-300 border-rose-800/60'
                    }`}>
                      {sess.status}
                    </span>
                  </td>
                  <td className="py-3.5 px-4 text-right text-[#8b949e]">
                    {sess.receiptCount.toLocaleString()}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Audit Event Stream Section */}
      <div className="space-y-4">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div>
            <h2 className="text-base font-bold text-[#e6edf3]">
              Cryptographic Audit Stream (Blake3 / Merkle)
            </h2>
            <p className="text-xs text-[#8b949e]">
              Tamper-evident logs recording every compilation, contractivity proof check, and runtime trip.
            </p>
          </div>

          {/* Filter Toolbar */}
          <div className="flex flex-wrap items-center gap-2">
            <div className="flex items-center rounded-lg bg-[#090d14] border border-[#233147] p-1 text-xs">
              {(['all', 'info', 'warning', 'violation'] as const).map((sev) => (
                <button
                  key={sev}
                  onClick={() => setSeverityFilter(sev)}
                  className={`px-2.5 py-1 rounded capitalize transition-colors cursor-pointer ${
                    severityFilter === sev
                      ? 'bg-[#1b2738] text-cyan-300 font-semibold'
                      : 'text-[#8b949e] hover:text-[#e6edf3]'
                  }`}
                >
                  {sev}
                </button>
              ))}
            </div>

            <div className="relative">
              <Search className="w-3.5 h-3.5 absolute left-3 top-1/2 -translate-y-1/2 text-[#6e7681]" />
              <input
                type="text"
                placeholder="Filter receipts..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-8 pr-3 py-1.5 bg-[#090d14] border border-[#233147] rounded-lg text-xs text-[#e6edf3] placeholder-[#6e7681] outline-none"
              />
            </div>
          </div>
        </div>

        {/* Audit Table */}
        <div className="border border-[#233147] rounded-xl bg-[#0e1624] overflow-x-auto">
          <table className="w-full text-left text-xs">
            <thead className="bg-[#090d14] text-[#8b949e] border-b border-[#233147] font-mono text-[11px]">
              <tr>
                <th className="py-3 px-4">Timestamp</th>
                <th className="py-3 px-4">Type</th>
                <th className="py-3 px-4">Artifact / Session</th>
                <th className="py-3 px-4">Receipt Hash</th>
                <th className="py-3 px-4">Spectral ρ</th>
                <th className="py-3 px-4">Status</th>
                <th className="py-3 px-4 text-right">Details</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-[#1e2c40]">
              {filteredEvents.map((evt) => (
                <tr key={evt.id} className="hover:bg-[#121c2d] transition-colors font-mono">
                  <td className="py-3.5 px-4 text-[#8b949e] whitespace-nowrap">
                    {evt.timestamp}
                  </td>
                  <td className="py-3.5 px-4 whitespace-nowrap">
                    <span className="font-bold text-[#e6edf3]">{evt.eventType}</span>
                  </td>
                  <td className="py-3.5 px-4 text-sky-400 whitespace-nowrap">
                    {evt.sessionOrArtifact}
                  </td>
                  <td className="py-3.5 px-4 text-cyan-300 max-w-xs truncate">
                    {evt.receiptHash}
                  </td>
                  <td className="py-3.5 px-4 font-bold whitespace-nowrap">
                    <span className={evt.spectralRadius < 1.0 ? 'text-emerald-400' : 'text-rose-400'}>
                      {evt.spectralRadius.toFixed(3)}
                    </span>
                  </td>
                  <td className="py-3.5 px-4 whitespace-nowrap">
                    <span className={`px-2 py-0.5 rounded text-[10px] font-bold border ${
                      evt.status === 'CERTIFIED'
                        ? 'bg-emerald-950/80 text-emerald-300 border-emerald-800/60'
                        : evt.status === 'MONITORED'
                        ? 'bg-amber-950/80 text-amber-300 border-amber-800/60'
                        : 'bg-rose-950/80 text-rose-300 border-rose-800/60'
                    }`}>
                      {evt.status}
                    </span>
                  </td>
                  <td className="py-3.5 px-4 text-right whitespace-nowrap">
                    <button
                      onClick={() => setSelectedEvent(evt)}
                      className="px-2.5 py-1 rounded bg-[#1b2738] hover:bg-[#25364e] text-cyan-300 text-[11px] font-medium border border-[#2d3f58] transition-colors cursor-pointer"
                    >
                      Inspect
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Audit Event Inspector Modal */}
      {selectedEvent && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-150">
          <div className="w-full max-w-lg bg-[#0e1624] border border-[#2d3f58] rounded-xl shadow-2xl overflow-hidden flex flex-col">
            <div className="px-6 py-4 bg-[#090d14] border-b border-[#233147] flex items-center justify-between">
              <div>
                <span className="text-xs font-mono font-bold text-cyan-400">AUDIT PROVENANCE RECEIPT</span>
                <h3 className="text-sm font-bold text-[#e6edf3] font-mono mt-0.5">{selectedEvent.id}</h3>
              </div>
              <button
                onClick={() => setSelectedEvent(null)}
                className="text-[#8b949e] hover:text-[#e6edf3]"
              >
                ✕
              </button>
            </div>

            <div className="p-6 space-y-3 font-mono text-xs">
              <div className="p-3 bg-[#090d14] rounded-lg border border-[#1e2a3c]">
                <span className="text-[#6e7681] text-[10px] block">Blake3 Proof Receipt Hash</span>
                <span className="text-cyan-300 font-bold break-all">{selectedEvent.receiptHash}</span>
              </div>

              <div className="grid grid-cols-2 gap-2 text-xs">
                <div className="p-2.5 bg-[#090d14] rounded border border-[#1e2a3c]">
                  <span className="text-[#6e7681] text-[10px] block">Event Type</span>
                  <span className="text-[#e6edf3] font-bold">{selectedEvent.eventType}</span>
                </div>
                <div className="p-2.5 bg-[#090d14] rounded border border-[#1e2a3c]">
                  <span className="text-[#6e7681] text-[10px] block">Measured ρ</span>
                  <span className={selectedEvent.spectralRadius < 1 ? 'text-emerald-400 font-bold' : 'text-rose-400 font-bold'}>
                    {selectedEvent.spectralRadius.toFixed(3)}
                  </span>
                </div>
              </div>

              <div className="p-3 bg-[#090d14] rounded-lg border border-[#1e2a3c]">
                <span className="text-[#6e7681] text-[10px] block mb-1">Diagnostic Log</span>
                <p className="text-[#8b949e] leading-relaxed">{selectedEvent.details}</p>
              </div>
            </div>

            <div className="px-6 py-3 bg-[#090d14] border-t border-[#233147] flex justify-end">
              <button
                onClick={() => setSelectedEvent(null)}
                className="px-4 py-1.5 rounded-lg bg-[#1b2738] hover:bg-[#25364e] text-xs font-semibold text-[#e6edf3] transition-colors"
              >
                Dismiss
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
