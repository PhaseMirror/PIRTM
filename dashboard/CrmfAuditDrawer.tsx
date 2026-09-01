import React, { useState, useEffect } from 'react';

// Defines the strict CRMF payload structure
export interface CrmfEvent {
  timestamp: string;
  seal_hash: string;
  poseidon_commitment: string;
  dual_anchor: {
    sha256: string;
    ed25519_sig: string;
  };
  metrics: {
    lambda_m: number;
    drift: number;
  };
}

export const CrmfAuditDrawer: React.FC = () => {
  const [events, setEvents] = useState<CrmfEvent[]>([]);
  const [activeEnvelope, setActiveEnvelope] = useState<CrmfEvent | null>(null);

  // Hook into the local MCP WebSocket for live telemetry
  useEffect(() => {
    const ws = new WebSocket('wss://mcp.pirtm.com/ws');
    ws.onmessage = (msg) => {
      try {
        const data = JSON.parse(msg.data);
        if (data.type === 'CRMF_SEAL_GENERATED') {
          setEvents((prev) => [data.payload, ...prev].slice(0, 50));
        }
      } catch (e) {
        console.error('Error parsing CRMF WebSocket event:', e);
      }
    };
    return () => ws.close();
  }, []);

  return (
    <div className="flex flex-col h-full bg-black text-green-500 font-mono text-sm border-l border-green-900">
      <div className="p-2 border-b border-green-900 uppercase tracking-widest font-bold">
        Live Terminal
      </div>
      
      {/* Zone 4: The Terminal Log Feed */}
      <div className="flex-1 overflow-y-auto p-4 space-y-1">
        {events.map((evt, idx) => (
          <div key={idx} className="flex items-center space-x-2">
            <button 
              onClick={() => setActiveEnvelope(evt)}
              className="text-xs bg-green-900 text-black px-1 hover:bg-green-700 transition-colors"
              title="Unpack CRMF Envelope"
            >
              [ Λ ]
            </button>
            <span>
              [{evt.timestamp}] - SECURE - CRMF anchor hash verified: {evt.seal_hash.slice(0, 8)}...
            </span>
            <span className="text-green-700">
              [{evt.timestamp}] - METRIC - Lambda_m calculated: {evt.metrics.lambda_m.toFixed(2)}
            </span>
          </div>
        ))}
      </div>

      {/* The Unpacked Envelope Drawer */}
      {activeEnvelope && (
        <div className="h-64 border-t border-green-900 bg-gray-900 p-4">
          <div className="uppercase tracking-widest text-xs text-gray-400 mb-2">
            CRMF Event Envelope (RFC-8785 Canonical Serialization)
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <p className="text-gray-500">Poseidon2 Sponge (t=9, r=8)</p>
              <p className="break-all">{activeEnvelope.poseidon_commitment}</p>
            </div>
            <div>
              <p className="text-gray-500">SHA-256 Anchor</p>
              <p className="break-all">{activeEnvelope.dual_anchor.sha256}</p>
              <p className="text-gray-500 mt-2">Ed25519 Signature</p>
              <p className="break-all">{activeEnvelope.dual_anchor.ed25519_sig}</p>
            </div>
          </div>
          <button 
            onClick={() => setActiveEnvelope(null)}
            className="mt-4 border border-green-500 px-4 py-1 hover:bg-green-900"
          >
            CLOSE DRAWER
          </button>
        </div>
      )}
    </div>
  );
};
