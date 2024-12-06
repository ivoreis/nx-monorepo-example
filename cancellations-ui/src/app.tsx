import { BrowserRouter, Routes, Route, useParams } from 'react-router-dom';

// @ts-expect-error: remote component
import UI, { supportedPlatforms } from 'remote/UI';

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/:platform/:id" element={<UIPage />} />
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  );
}

function UIPage() {
  const { platform, id } = useParams();

  if (!platform || !supportedPlatforms.includes(platform)) {
    return <NotFound />;
  }

  return (
    <div>
      <h1>Host Cancellation Page</h1>
      <UI platform={platform} id={id} />
    </div>
  );
}

function NotFound() {
  return (
    <div>
      <h1>404 - Page Not Found</h1>
      <p>Please check your URL and try again. </p>
      <p>This UI expect the url in the format: `/:platform/:id`</p>
    </div>
  );
}
