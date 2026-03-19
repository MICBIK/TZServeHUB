import { BrowserRouter, Routes, Route } from 'react-router-dom';
import AppLayout from './components/layout/AppLayout';
import DashboardPage from './pages/DashboardPage';
import CpuPage from './pages/CpuPage';
import NetworkPage from './pages/NetworkPage';
import DiskPage from './pages/DiskPage';
import ProbePage from './pages/ProbePage';
import AlertPage from './pages/AlertPage';
import SettingsPage from './pages/SettingsPage';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AppLayout />}>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/cpu" element={<CpuPage />} />
          <Route path="/network" element={<NetworkPage />} />
          <Route path="/disk" element={<DiskPage />} />
          <Route path="/probes" element={<ProbePage />} />
          <Route path="/alerts" element={<AlertPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
