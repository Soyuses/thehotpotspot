// Video Dashboard entry point
import { apiClient } from './services/api';

class VideoApp {
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

      // Initialize video dashboard
      await this.render();

      console.log('Video dashboard initialized successfully');
    } catch (error) {
      console.error('Failed to initialize video app:', error);
      this.showError('Ошибка инициализации панели видеонаблюдения');
    }
  }

  private async render(): Promise<void> {
    this.container.innerHTML = `
      <div class="video-dashboard">
        <header class="dashboard-header">
          <h1>📹 Панель видеонаблюдения</h1>
          <div class="header-actions">
            <button class="btn btn-secondary" id="refreshBtn">🔄 Обновить</button>
            <button class="btn btn-primary" id="addStreamBtn">➕ Добавить поток</button>
          </div>
        </header>

        <main class="dashboard-content">
          <div class="video-grid" id="videoGrid">
            <div class="loading">Загрузка видеопотоков...</div>
          </div>
        </main>
      </div>
    `;

    await this.loadData();
    this.attachEventListeners();
  }

  private async loadData(): Promise<void> {
    try {
      const streams = await apiClient.getVideoStreams();
      this.renderVideoStreams(streams);
    } catch (error) {
      console.error('Error loading video data:', error);
      this.showError('Ошибка загрузки видеопотоков');
    }
  }

  private renderVideoStreams(streams: any[]): void {
    const container = document.getElementById('videoGrid');
    if (!container) return;

    if (streams.length === 0) {
      container.innerHTML = `
        <div class="no-streams">
          <h3>Нет активных видеопотоков</h3>
          <p>Добавьте новый поток для начала видеонаблюдения</p>
          <button class="btn btn-primary" onclick="videoApp.addStream()">
            ➕ Добавить поток
          </button>
        </div>
      `;
      return;
    }

    container.innerHTML = streams.map(stream => `
      <div class="video-card" data-stream-id="${stream.stream_id}">
        <div class="video-header">
          <h3>${stream.name}</h3>
          <div class="video-actions">
            <button class="btn btn-sm ${stream.is_active ? 'btn-danger' : 'btn-success'}" 
                    onclick="videoApp.toggleStream('${stream.stream_id}')">
              ${stream.is_active ? '⏸️ Остановить' : '▶️ Запустить'}
            </button>
            <button class="btn btn-sm btn-secondary" onclick="videoApp.editStream('${stream.stream_id}')">
              ✏️ Редактировать
            </button>
          </div>
        </div>
        <div class="video-content">
          <div class="video-placeholder">
            <div class="video-icon">📹</div>
            <p>${stream.source.type === 'camera' ? 'Камера' : 'YouTube'}: ${stream.source.value}</p>
            <div class="video-status ${stream.is_active ? 'active' : 'inactive'}">
              ${stream.is_active ? 'Активен' : 'Неактивен'}
            </div>
          </div>
        </div>
        <div class="video-info">
          <small>Создан: ${new Date(stream.created_at).toLocaleDateString()}</small>
        </div>
      </div>
    `).join('');
  }

  private attachEventListeners(): void {
    document.getElementById('refreshBtn')?.addEventListener('click', () => {
      this.loadData();
    });

    document.getElementById('addStreamBtn')?.addEventListener('click', () => {
      this.addStream();
    });
  }

  public addStream(): void {
    const name = prompt('Название потока:');
    if (!name) return;

    const sourceType = prompt('Тип источника (camera/youtube):');
    if (!sourceType) return;

    const sourceValue = prompt('Значение источника:');
    if (!sourceValue) return;

    this.createStream(name, sourceType, sourceValue);
  }

  private async createStream(name: string, sourceType: string, sourceValue: string): Promise<void> {
    try {
      const stream = await apiClient.createVideoStream(name, sourceType, sourceValue);
      if (stream) {
        this.showSuccess('Поток создан успешно');
        await this.loadData();
      } else {
        this.showError('Ошибка создания потока');
      }
    } catch (error) {
      this.showError('Ошибка создания потока');
    }
  }

  public async toggleStream(streamId: string): Promise<void> {
    try {
      // Find stream and toggle its status
      const streamElement = document.querySelector(`[data-stream-id="${streamId}"]`);
      if (!streamElement) return;

      const isActive = streamElement.querySelector('.video-status')?.classList.contains('active');
      
      if (isActive) {
        await apiClient.deactivateVideoStream(streamId);
      } else {
        await apiClient.activateVideoStream(streamId);
      }
      
      await this.loadData();
    } catch (error) {
      this.showError('Ошибка изменения статуса потока');
    }
  }

  public editStream(streamId: string): void {
    alert('Функция редактирования потока будет реализована');
  }

  private showError(message: string): void {
    console.error(message);
    alert(message);
  }

  private showSuccess(message: string): void {
    console.log(message);
    alert(message);
  }
}

// Initialize app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  const app = new VideoApp();
  // Make app globally available
  (window as any).videoApp = app;
});
