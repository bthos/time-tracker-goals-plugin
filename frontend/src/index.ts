// Frontend entry point for Goals plugin
// This file registers plugin components with the core app

import type { PluginFrontendModule, PluginFrontendAPI } from './types';
import { Goals } from './components/Goals';
import GoalProgressWidget from './components/GoalProgressWidget';

const initialize: PluginFrontendModule['initialize'] = (api: PluginFrontendAPI) => {
  // Register settings tab
  api.registerSettingsTab({
    id: 'goals',
    label: 'Goals',
    component: Goals,
    order: 5,
  });
  
  // Register dashboard widgets
  api.registerDashboardWidget({
    id: 'goal-progress-widget',
    component: GoalProgressWidget,
    order: 45,
    gridColSpan: 1,
  });
  
  // Note: GoalAlerts is already imported in Dashboard, so we don't need to register it separately
  // It's used directly in the core Dashboard component
};

const module: PluginFrontendModule = {
  initialize,
};

export default module;
