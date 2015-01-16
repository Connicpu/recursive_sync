#include <stdlib.h>
#include <stdbool.h>

#ifdef __WIN32__

#define WIN32_LEAN_AND_MEAN
#include <windows.h>

#define RSMUTEX_SPIN_COUNT 0x00000400

#else

#include <pthread.h>

#endif

struct rs_mutex {

  #ifdef __WIN32__

  CRITICAL_SECTION csec;

  #else

  pthread_mutex_t mutex;
  pthread_mutexattr_t attr;

  #endif

};

struct rs_mutex *rs_mutex_alloc() {
  struct rs_mutex *rmtx = (struct rs_mutex *) malloc(sizeof(struct rs_mutex));
  if (rmtx == NULL) return NULL;

  #ifdef __WIN32__

  if (!InitializeCriticalSectionAndSpinCount(&rmtx->csec, RSMUTEX_SPIN_COUNT)) {
    printf("InitializeCriticalSectionAndSpinCount failed\nGetLastError(): %x\n", GetLastError());
    free(rmtx);
    return NULL;
  }

  #else

  if (pthread_mutexattr_init(&rmtx->attr)) {
    return NULL;
  }
  if (pthread_mutexattr_settype(&rmtx->attr, PTHREAD_MUTEX_RECURSIVE)) {
    pthread_mutexattr_destroy(&rmtx->attr);
    return NULL;
  }
  if (pthread_mutex_init(&rmtx->mutex, &rmtx->attr)) {
    pthread_mutexattr_destroy(&rmtx->attr);
    return NULL;
  }

  #endif

  return rmtx;
}

void rs_mutex_acquire(struct rs_mutex *rmtx) {
  #ifdef __WIN32__

  EnterCriticalSection(&rmtx->csec);

  #else

  pthread_mutex_lock(&rmtx->mutex);

  #endif
}

void rs_mutex_release(struct rs_mutex *rmtx) {
  #ifdef __WIN32__

  LeaveCriticalSection(&rmtx->csec);

  #else

  pthread_mutex_unlock(&rmtx->mutex);

  #endif
}

void rs_mutex_free(struct rs_mutex *rmtx) {
  #ifdef __WIN32__

  DeleteCriticalSection(&rmtx->csec);

  #else

  pthread_mutexattr_destroy(&rmtx->attr);
  pthread_mutex_destroy(&rmtx->mutex);

  #endif

  free(rmtx);
}
