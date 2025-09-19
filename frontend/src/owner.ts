// Owner Dashboard entry point
import { OwnerDashboard } from './components/OwnerDashboard';
import { apiClient } from './services/api';

class OwnerApp {
  private container: HTMLElement;
  private dashboard: OwnerDashboard | null = null;

  constructor() {
    this.container = document.getElementById('app')!;
    this.init();
  }

  private async init(): Promise<void> {
    try {
      // Check API health
      const isHealthy = await apiClient.healthCheck();
      if (!isHealthy) {
        this.showError('API сервер недоступен. Проверьте подключение.');
        return;
      }

      // Initialize owner dashboard
      this.dashboard = new OwnerDashboard(this.container);
      await this.dashboard.render();

      console.log('Owner dashboard initialized successfully');
    } catch (error) {
      console.error('Failed to initialize owner app:', error);
      this.showError('Ошибка инициализации панели владельца');
    }
  }

  private showError(message: string): void {
    this.container.innerHTML = `
      <div class="error-page">
        <div class="error-content">
          <h1>⚠️ Ошибка</h1>
          <p>${message}</p>
          <button class="btn btn-primary" onclick="location.reload()">
            🔄 Перезагрузить страницу
          </button>
          <a href="index.html" class="btn btn-secondary" style="margin-left: 1rem;">
            🏠 На главную
          </a>
        </div>
      </div>
    `;
  }
}

// Initialize app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  new OwnerApp();
});

// Export for global access
(window as any).OwnerApp = OwnerApp;
