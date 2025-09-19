// Franchise Dashboard entry point
import { FranchiseDashboard } from './components/FranchiseDashboard';
import { apiClient } from './services/api';

class FranchiseApp {
  private container: HTMLElement;
  private dashboard: FranchiseDashboard | null = null;

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

      // Initialize franchise dashboard
      this.dashboard = new FranchiseDashboard(this.container);
      await this.dashboard.render();

      console.log('Franchise dashboard initialized successfully');
    } catch (error) {
      console.error('Failed to initialize franchise app:', error);
      this.showError('Ошибка инициализации панели франчайзи');
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

  // Public methods for order actions
  public async confirmOrder(orderId: string): Promise<void> {
    if (this.dashboard) {
      await this.dashboard.confirmOrder(orderId);
    }
  }

  public async cancelOrder(orderId: string): Promise<void> {
    if (this.dashboard) {
      await this.dashboard.cancelOrder(orderId);
    }
  }

  public startCooking(orderId: string): void {
    if (this.dashboard) {
      this.dashboard.startCooking(orderId);
    }
  }

  public finishCooking(orderId: string): void {
    if (this.dashboard) {
      this.dashboard.finishCooking(orderId);
    }
  }

  public deliverOrder(orderId: string): void {
    if (this.dashboard) {
      this.dashboard.deliverOrder(orderId);
    }
  }

  public editMenuItem(itemId: string): void {
    if (this.dashboard) {
      this.dashboard.editMenuItem(itemId);
    }
  }

  public deleteMenuItem(itemId: string): void {
    if (this.dashboard) {
      this.dashboard.deleteMenuItem(itemId);
    }
  }
}

// Initialize app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  const app = new FranchiseApp();
  // Make app globally available for onclick handlers
  (window as any).franchiseApp = app;
});

// Export for global access
(window as any).FranchiseApp = FranchiseApp;
