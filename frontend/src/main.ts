// Main entry point for The Hot Pot Spot frontend
import { MainPage } from './components/MainPage';
import { apiClient } from './services/api';

class App {
  private container: HTMLElement;
  private currentPage: MainPage | null = null;

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

      // Initialize main page
      this.currentPage = new MainPage(this.container);
      await this.currentPage.render();

      // Setup global error handling
      this.setupErrorHandling();

      console.log('The Hot Pot Spot frontend initialized successfully');
    } catch (error) {
      console.error('Failed to initialize app:', error);
      this.showError('Ошибка инициализации приложения');
    }
  }

  private setupErrorHandling(): void {
    window.addEventListener('error', (event) => {
      console.error('Global error:', event.error);
      this.showError('Произошла неожиданная ошибка');
    });

    window.addEventListener('unhandledrejection', (event) => {
      console.error('Unhandled promise rejection:', event.reason);
      this.showError('Ошибка выполнения запроса');
    });
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
        </div>
      </div>
    `;
  }
}

// Initialize app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  new App();
});

// Export for global access
(window as any).App = App;
