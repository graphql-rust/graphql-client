export function convenient_post(url, body, onSuccess, onFailure) {
  return fetch(url, {
    body,
    method: "POST",
    headers: {
      ["Content-Type"]: "application/json",
      ["Accept"]: "application/json"
    }
  })
    .then(res => res.text())
    .then(onSuccess)
    .catch(f => onSuccess(f.toString()));
  // .catch(onFailure);
}
