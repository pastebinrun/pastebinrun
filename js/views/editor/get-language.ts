// pastebin.run
// Copyright (C) 2020 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

const cache = new Map();

async function fetchLanguage(identifier: string) {
  const response = await fetch(`/api/v0/language/${identifier}`);
  return await response.json();
}

export default async function getLanguage(
  identifier: string,
  shouldRetry: () => boolean
) {
  if (cache.has(identifier)) {
    return cache.get(identifier);
  }
  while (shouldRetry()) {
    try {
      const response = await fetchLanguage(identifier);
      cache.set(identifier, response);
      return response;
    } catch (e) {
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }
  await new Promise(() => {});
}
