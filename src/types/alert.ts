export interface AlertRule {
  id: string;
  server_id: string | null;
  metric_key: string;
  condition: 'gt' | 'lt' | 'eq';
  threshold: number;
  duration_sec: number;
  cooldown_sec: number;
  enabled: boolean;
}

export interface AlertEvent {
  id: number;
  rule_id: string;
  server_id: string;
  metric_key: string;
  value: number;
  status: 'firing' | 'resolved';
  fired_at: number;
  resolved_at: number | null;
}
