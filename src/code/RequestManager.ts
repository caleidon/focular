async function findBestYoutubeUrl(url: string): Promise<string> {
  let foundUrl: string;
  let videoId: string = youtubeUrlToId(url);
  let dataEndpoint: string = buildVideoDataEndpoint(videoId);

  await fetch(dataEndpoint)
    .then(function (response) {
      return response.json();
    })
    .then(function (responseJson) {
      if (
        responseJson.data.player_response.streamingData.hasOwnProperty(
          "adaptiveFormats"
        )
      ) {
        let streams = [];

        if (
          responseJson.data.player_response.streamingData &&
          responseJson.data.player_response.streamingData.formats
        ) {
          streams = streams.concat(
            responseJson.data.player_response.streamingData.formats
          );
        }

        if (
          responseJson.data.player_response.streamingData &&
          responseJson.data.player_response.streamingData.adaptiveFormats
        ) {
          streams = streams.concat(
            responseJson.data.player_response.streamingData.adaptiveFormats
          );
        }

        foundUrl = findBestQualityStreamUrl(streams);
      }
    });

  return foundUrl;
}

function youtubeUrlToId(url: string): string {
  const regex =
    /^(?:http(?:s)?:\/\/)?(?:www\.)?(?:m\.)?(?:youtu\.be\/|(?:(?:youtube-nocookie\.com\/|youtube\.com\/)(?:(?:watch)?\?(?:.*&)?v(?:i)?=|(?:embed|v|vi|user)\/)))([a-zA-Z0-9\-_]*)/;
  const matches = url.match(regex);
  return Array.isArray(matches) && matches[1] ? matches[1] : url;
}

function buildVideoDataEndpoint(videoId: string): string {
  return `https://yt2html5.com/?id=${videoId}`;
}

function findBestQualityStreamUrl(streams): string {
  let foundUrl: string = "error";
  streams.forEach((stream) => {
    if (
      stream.mimeType.includes("mp4") &&
      "audioQuality" in stream &&
      stream.audioQuality &&
      stream.qualityLabel
    ) {
      if (stream.qualityLabel.includes("720p")) {
        foundUrl = stream.url;
      } else if (stream.qualityLabel.includes("480p")) {
        foundUrl = stream.url;
      } else if (stream.qualityLabel.includes("360p")) {
        foundUrl = stream.url;
      } else if (stream.qualityLabel.includes("240p")) {
        foundUrl = stream.url;
      }
    }
  });

  return foundUrl;
}

export { findBestYoutubeUrl as getDirectYoutubeUrl };
