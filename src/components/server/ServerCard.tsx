import { ServerConfig } from '../../types/server';

interface ServerCardProps {
  server: ServerConfig;
  isActive: boolean;
  onClick: () => void;
}

export default function ServerCard({ server, isActive, onClick }: ServerCardProps) {
  return (
    <div
      onClick={onClick}
      className={`p-4 rounded-lg border cursor-pointer transition-colors ${
        isActive
          ? 'bg-blue-900/20 border-blue-500'
          : 'bg-gray-800 border-gray-700 hover:border-gray-600'
      }`}
    >
      <div className="flex items-center justify-between mb-2">
        <h3 className="font-semibold text-white">{server.name}</h3>
        <span
          className={`w-2 h-2 rounded-full ${
            server.enabled ? 'bg-green-500' : 'bg-gray-500'
          }`}
        />
      </div>
      <p className="text-sm text-gray-400">{server.host}:{server.port}</p>
      <p className="text-xs text-gray-500 mt-1">{server.adapter_type}</p>
    </div>
  );
}
