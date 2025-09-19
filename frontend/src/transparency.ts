// Transparency Dashboard entry point
import { apiClient } from './services/api';

class TransparencyApp {
  private container: HTMLElement;

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

      // Initialize transparency dashboard
      await this.render();

      console.log('Transparency dashboard initialized successfully');
    } catch (error) {
      console.error('Failed to initialize transparency app:', error);
      this.showError('Ошибка инициализации панели прозрачности');
    }
  }

  private async render(): Promise<void> {
    this.container.innerHTML = `
      <div class="transparency-dashboard">
        <header class="dashboard-header">
          <h1>🔍 Панель прозрачности</h1>
          <div class="header-actions">
            <button class="btn btn-secondary" id="refreshBtn">🔄 Обновить</button>
            <button class="btn btn-primary" id="exportBtn">📊 Экспорт данных</button>
          </div>
        </header>

        <main class="dashboard-content">
          <div class="transparency-grid">
            <div class="card">
              <h3>📈 Отчеты по эмиссии</h3>
              <div id="emissionsReport">
                <p>Загрузка отчетов...</p>
              </div>
            </div>
            
            <div class="card">
              <h3>👥 Держатели токенов</h3>
              <div id="holdersReport">
                <p>Загрузка данных о держателях...</p>
              </div>
            </div>
            
            <div class="card">
              <h3>💰 Финансовые отчеты</h3>
              <div id="financialReport">
                <p>Загрузка финансовых данных...</p>
              </div>
            </div>
          </div>
        </main>
      </div>
    `;

    await this.loadData();
    this.attachEventListeners();
  }

  private async loadData(): Promise<void> {
    try {
      // Load transparency data
      await this.loadEmissionsReport();
      await this.loadHoldersReport();
      await this.loadFinancialReport();
    } catch (error) {
      console.error('Error loading transparency data:', error);
      this.showError('Ошибка загрузки данных');
    }
  }

  private async loadEmissionsReport(): Promise<void> {
    const container = document.getElementById('emissionsReport');
    if (!container) return;

    // Mock data for now
    container.innerHTML = `
      <div class="report-content">
        <div class="report-item">
          <span>Security Tokens (ST):</span>
          <span class="value">1,250,000 THP</span>
        </div>
        <div class="report-item">
          <span>Utility Tokens (SPOT):</span>
          <span class="value">450,000 SPOT</span>
        </div>
        <div class="report-item">
          <span>Общая эмиссия:</span>
          <span class="value">1,700,000 токенов</span>
        </div>
      </div>
    `;
  }

  private async loadHoldersReport(): Promise<void> {
    const container = document.getElementById('holdersReport');
    if (!container) return;

    // Mock data for now
    container.innerHTML = `
      <div class="report-content">
        <div class="report-item">
          <span>Активные держатели ST:</span>
          <span class="value">1,250</span>
        </div>
        <div class="report-item">
          <span>Активные держатели SPOT:</span>
          <span class="value">3,450</span>
        </div>
        <div class="report-item">
          <span>Всего участников:</span>
          <span class="value">4,700</span>
        </div>
      </div>
    `;
  }

  private async loadFinancialReport(): Promise<void> {
    const container = document.getElementById('financialReport');
    if (!container) return;

    // Mock data for now
    container.innerHTML = `
      <div class="report-content">
        <div class="report-item">
          <span>Общая выручка:</span>
          <span class="value">125,000 GEL</span>
        </div>
        <div class="report-item">
          <span>Средний чек:</span>
          <span class="value">25.50 GEL</span>
        </div>
        <div class="report-item">
          <span>Количество транзакций:</span>
          <span class="value">4,900</span>
        </div>
      </div>
    `;
  }

  private attachEventListeners(): void {
    document.getElementById('refreshBtn')?.addEventListener('click', () => {
      this.loadData();
    });

    document.getElementById('exportBtn')?.addEventListener('click', () => {
      this.exportData();
    });
  }

  private exportData(): void {
    // Implement data export
    alert('Функция экспорта данных будет реализована');
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
  new TransparencyApp();
});
