import { render, screen } from '@testing-library/react';
import App from '../src/App';

describe('App', () => {
  test('renders documentation header', () => {
    render(<App />);
    // Check for loading state initially or app content
    const loadingElement = screen.getByText(/Loading documentation/i);
    expect(loadingElement).toBeInTheDocument();
  });

  test('renders after data loads', async () => {
    render(<App />);
    // The app shows sample data after load
    const header = await screen.findByText(/Automated Document Synthesizer/i);
    expect(header).toBeInTheDocument();
  });

  test('renders navigation tabs', async () => {
    render(<App />);
    const overviewTab = await screen.findByText(/Overview/i);
    expect(overviewTab).toBeInTheDocument();
    const modulesTab = await screen.findByText(/Modules/i);
    expect(modulesTab).toBeInTheDocument();
    const apiTab = await screen.findByText(/API/i);
    expect(apiTab).toBeInTheDocument();
    const architectureTab = await screen.findByText(/Architecture/i);
    expect(architectureTab).toBeInTheDocument();
    const statisticsTab = await screen.findByText(/Statistics/i);
    expect(statisticsTab).toBeInTheDocument();
  });
});
